#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

mod app;

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();
    
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([450.0, 960.0])
            .with_min_inner_size([540.0, 960.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "headline App",
        native_options,
        //Box::new(|cc| Box::new(app::TemplateApp::new(cc))),
        Box::new(|cc| Box::new(app::Stocks::new(cc)))
    )
}