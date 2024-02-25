use egui::ViewportCommand;

pub fn toggle_fullscreen(ctx: &egui::Context) {
    if let Some(fullscreen) = ctx.input_mut(|i| i.viewport().fullscreen) {
        ctx.send_viewport_cmd(ViewportCommand::Fullscreen(!fullscreen));
    }
}
