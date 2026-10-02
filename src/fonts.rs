use eframe::egui;

pub(crate) fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "noto_sans_jp".to_owned(),
        egui::FontData::from_static(include_bytes!("../vendor/fonts/NotoSansCJKjp-Regular.otf"))
            .into(),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "noto_sans_jp".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("noto_sans_jp".to_owned());

    ctx.set_fonts(fonts);
}
