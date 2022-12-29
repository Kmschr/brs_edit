use std::sync::Arc;
use eframe::egui_wgpu::{
    self,
    wgpu,
};
use crate::EditorApp;

impl EditorApp {
    pub fn _render_ui(&mut self, ui: &mut egui::Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            self._custom_painting(ui);
        });
    }
}

impl EditorApp {
    fn _custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(egui::Vec2::splat(800.0), egui::Sense::drag());
        self._angle += response.drag_delta().x * 0.01;

        // Clone locals so we can move them into the paint callback:
        let angle = self._angle;

        // The callback function for WGPU is in two stages: prepare, and paint.
        //
        // The prepare callback is called every frame before paint and is given access to the wgpu
        // Device and Queue, which can be used, for instance, to update buffers and uniforms before
        // rendering.
        //
        // You can use the main `CommandEncoder` that is passed-in, return an arbitrary number of
        // user-defined `CommandBuffer`s, or both. The main command buffer, as well as all
        // user-defined ones, will be submitted together to the GPU in a single call.
        //
        // The paint callback is called after prepare and is given access to the render pass, which
        // can be used to issue draw commands.
        let cb = egui_wgpu::CallbackFn::new().prepare(move |device, queue, _encoder, paint_callback_resources| {
            let resources: &TriangleRenderResources = paint_callback_resources.get().unwrap();
            resources._prepare(device, queue, angle);
            Vec::new()
        }).paint(move |_info, render_pass, paint_callback_resources| {
            let resources: &TriangleRenderResources = paint_callback_resources.get().unwrap();
            resources._paint(render_pass);
        });
        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(cb),
        };
        ui.painter().add(callback);
    }
}

pub struct TriangleRenderResources {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
}

impl TriangleRenderResources {
    fn _prepare(&self, _device: &wgpu::Device, queue: &wgpu::Queue, angle: f32) {
        // Update our uniform buffer with the angle from the UI
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[angle, 0.0, 0.0, 0.0]));
    }

    fn _paint<'rp>(&'rp self, render_pass: &mut wgpu::RenderPass<'rp>) {
        // Draw our triangle!
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0 .. 3, 0 .. 1);
    }
}
