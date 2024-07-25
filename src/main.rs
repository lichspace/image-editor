use eframe::egui::{self};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "MyEguiApp",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

#[derive(Default)]
struct MyEguiApp {
    picked_path: Option<String>,
    texture: Option<egui::TextureHandle>,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            if let Some(picked_path) = &self.picked_path {
                if self.texture.as_ref().unwrap().name() != picked_path.as_str() {
                    let cimg: egui::ColorImage = load_image_from_path(picked_path).unwrap();
                    self.texture =
                        Some(ui.ctx().load_texture(picked_path, cimg, Default::default()));
                }
            }

            if let Some(texture) = self.texture.as_ref() {
                ui.image((texture.id(), ui.available_size()));
            } else {
                ui.spinner();
            }
        });
    }
}

fn load_image_from_path(path: &str) -> Result<egui::ColorImage, image::ImageError> {
    println!("{}", path);
    let image = image::io::Reader::open(path)?.decode()?;
    // image.resize(800, 400, FilterType::Nearest);
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
