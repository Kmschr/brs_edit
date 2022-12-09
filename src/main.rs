#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod editor;
mod explorer;
mod file_dialog;
mod gui;
mod icon;
mod menu;
mod open;
mod render;
mod save;
mod shortcuts;
mod view;

use eframe::egui;
use eframe::egui_wgpu::wgpu;
use eframe::wgpu::util::DeviceExt;
use egui::*;
use file_dialog::default_build_directory;
use render::TriangleRenderResources;
use std::num::NonZeroU64;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

const DEFAULT_WINDOW_SIZE: Vec2 = Vec2::new(1920.0, 1080.0);

// TODO:
// - See if a file was modified
// - Shortcuts for saving
// - Save As
// - Render preview (GLOW) ?
// - Import > BLS
// - Import > OBJ
// - Import > io (Bricklink Studio)

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(DEFAULT_WINDOW_SIZE),
        icon_data: Some(eframe::IconData {
            rgba: icon::ICON.to_vec(),
            width: 32,
            height: 32,
        }),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "BRS Editor",
        native_options,
        Box::new(|cc| Box::new(EditorApp::new(cc))),
    )
}

struct EditorApp {
    angle: f32,
    default_build_dir: Option<PathBuf>,
    default_documents_dir: Option<PathBuf>,
    default_downloads_dir: Option<PathBuf>,
    file_path: Option<PathBuf>,
    folder_path: Option<PathBuf>,
    receivers: Receivers,
    save_data: Option<brickadia::save::SaveData>,
    save_colors: Vec<([f32; 4], u32)>,
    preview_handle: Option<TextureHandle>,
}

struct Receivers {
    file_path_receiver: Option<Receiver<Option<PathBuf>>>,
    folder_path_receiver: Option<Receiver<Option<PathBuf>>>,
    preview_path_receiver: Option<Receiver<Option<PathBuf>>>,
    save_as_path_receiever: Option<Receiver<Option<PathBuf>>>,
}

impl Receivers {
    fn new() -> Self {
        Self {
            file_path_receiver: None,
            folder_path_receiver: None,
            preview_path_receiver: None,
            save_as_path_receiever: None,
        }
    }
}

impl EditorApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref();

        if let Some(wgpu_render_state) = wgpu_render_state {
            let device = &wgpu_render_state.device;
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("custom3d"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("./custom3d_wgpu_shader.wgsl").into(),
                ),
            });

            let bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("custom3d"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: NonZeroU64::new(16),
                        },
                        count: None,
                    }],
                });

            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("custom3d"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("custom3d"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu_render_state.target_format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

            let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("custom3d"),
                contents: bytemuck::cast_slice(&[0.0_f32; 4]), // 16 bytes aligned!
                // Mapping at creation (as done by the create_buffer_init utility) doesn't require us to to add the MAP_WRITE usage
                // (this *happens* to workaround this bug )
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            });

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("custom3d"),
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });

            wgpu_render_state
                .renderer
                .write()
                .paint_callback_resources
                .insert(TriangleRenderResources {
                    pipeline,
                    bind_group,
                    uniform_buffer,
                });
        }

        Self {
            angle: 0.0,
            default_build_dir: default_build_directory(),
            default_documents_dir: dirs::document_dir(),
            default_downloads_dir: dirs::download_dir(),
            file_path: None,
            folder_path: None,
            receivers: Receivers::new(),
            save_data: None,
            save_colors: vec![],
            preview_handle: None,
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.handle_shortcuts(ctx, frame);
        self.receive_file_dialog_paths(ctx);

        TopBottomPanel::top("menu_panel")
            .frame(gui::TOP_FRAME)
            .show(ctx, |ui| {
                self.show_menu(ui, ctx, frame);
            });
        TopBottomPanel::bottom("info_panel")
            .frame(gui::BOTTOM_FRAME)
            .show(ctx, |ui| {
                self.bottom_panel(ui);
            });
        SidePanel::left("file_panel")
            .resizable(true)
            .frame(gui::LEFT_FRAME)
            .max_width(DEFAULT_WINDOW_SIZE.x / 2.0)
            .show(ctx, |ui| {
                self.explorer_ui(ui, ctx);
            });
        // SidePanel::right("render_panel")
        //     .resizable(false)
        //     .frame(gui::RIGHT_FRAME)
        //     .max_width(DEFAULT_WINDOW_SIZE.x / 2.0)
        //     .show(ctx, |ui| {
        //         self.render_ui(ui);
        //     });
        CentralPanel::default()
            .frame(gui::CENTER_FRAME)
            .show(ctx, |ui| {
                if self.file_path.is_none() {
                    self.starting_page(ui);
                } else if self.save_data.is_some() {
                    ScrollArea::vertical().stick_to_right(true).show(ui, |ui| {
                        self.editor_ui(ui);
                    });
                }
            });
    }
}

impl EditorApp {
    fn bottom_panel(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if let Some(file_path) = &self.file_path {
                ui.strong(file_path.to_string_lossy());
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                ui.add_space(5.0);
                let icon = RichText::new("\u{e624}").strong();
                ui.visuals_mut().hyperlink_color = Color32::WHITE;
                ui.hyperlink_to(icon, "https://github.com/Kmschr/brs_edit");
            });
        });
    }

    fn starting_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("BRS Editor");
        gui::header(ui, "Start");
        ui.add_space(5.0);
        if ui.link("🗋 Open File...").clicked() {
            self.choose_file();
        }
        ui.add_space(5.0);
        if ui.link("🗁 Open Folder...").clicked() {
            self.choose_folder();
        }
        ui.add_space(5.0);
        gui::header(ui, "Common Places");
        ui.add_space(5.0);
        if let Some(default_build_dir) = &self.default_build_dir {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                if ui.link("Builds").clicked() {
                    self.folder_path = Some(default_build_dir.clone());
                };
                ui.strong(default_build_dir.to_string_lossy());
            });
        }
        if let Some(default_documents_dir) = &self.default_documents_dir {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                if ui.link("Documents").clicked() {
                    self.folder_path = Some(default_documents_dir.clone());
                };
                ui.strong(default_documents_dir.to_string_lossy());
            });
        }
        if let Some(default_downloads_dir) = &self.default_downloads_dir {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                if ui.link("Downloads").clicked() {
                    self.folder_path = Some(default_downloads_dir.clone());
                };
                ui.strong(default_downloads_dir.to_string_lossy());
            });
        }
    }
}
