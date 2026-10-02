use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

const SELECTION_MARKDOWN: &str = include_str!("../tests/fixtures/selection.md");

#[derive(Default)]
pub(crate) struct ScrapViewerApp {
    markdown_cache: CommonMarkCache,
}

impl eframe::App for ScrapViewerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("スクラップビューアー");
            ui.separator();

            ui.style_mut().interaction.selectable_labels = true;
            ui.style_mut().interaction.multi_widget_text_select = true;

            egui::ScrollArea::both().show(ui, |ui| {
                CommonMarkViewer::new().show(ui, &mut self.markdown_cache, SELECTION_MARKDOWN);
            });
        });
    }
}
