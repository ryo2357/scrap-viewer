mod app;
mod fonts;

fn main() -> eframe::Result {
    eframe::run_native(
        "Scrap Viewer",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            fonts::setup_fonts(&cc.egui_ctx);
            Ok(Box::new(app::ScrapViewerApp::default()))
        }),
    )
}
