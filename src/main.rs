mod app;

fn main() {
    let option = eframe::NativeOptions {
        default_theme:eframe::Theme::Light,
        ..Default::default()
    };
    eframe::run_native("Todo", option, Box::new(|cc|Box::new(app::MyApp::new(cc))));
}
