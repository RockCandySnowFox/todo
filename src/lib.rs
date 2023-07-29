pub mod todolist;
pub fn setup_custom_fonts(ctx: &eframe::egui::Context) {
    let mut fonts = eframe::egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../SmileySans-Oblique.ttf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(eframe::egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(eframe::egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
    use eframe::egui::FontFamily::Proportional;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            eframe::egui::TextStyle::Heading,
            eframe::egui::FontId::new(66.0, Proportional),
        ),
        (eframe::egui::TextStyle::Body, eframe::egui::FontId::new(16.0, Proportional)),
        (
            eframe::egui::TextStyle::Monospace,
            eframe::egui::FontId::new(12.0, Proportional),
        ),
        (
            eframe::egui::TextStyle::Button,
            eframe::egui::FontId::new(16.0, Proportional),
        ),
        (eframe::egui::TextStyle::Small, eframe::egui::FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}