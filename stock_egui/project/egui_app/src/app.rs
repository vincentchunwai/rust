use egui::{FontDefinitions, Label};
use serde::{Serialize, Deserialize};
use finnhub_api::{Quote, FinnhubApi};
pub const PADDING: f32 = 5.0;
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const CYAN: Color32 = Color32::from_rgb(0, 200, 221);
pub const BLACK: Color32 = Color32::from_rgb(0, 0, 0);


pub struct Stocks{
    stocks: Vec<StocksCardData>
    config: API_Config,
}

#[derive(Serialize, Deserialize)]
pub struct API_Config{
    pub token: String,
    pub token_initialize: bool,
}

impl Default for API_Config{
    fn default() -> Self {
        Self{
            token:: String::new(),
            token_initialize:: false,
        }
    }
}

struct StocksCardData{
    pub symbol: String,
    pub c: f64,  // Current price
    pub d: f64,  // Change
    pub dp: f64, // Percent change
    pub h: f64,  // Day high
    pub l: f64,  // Day low
    pub o: f64,  // Open price
    pub pc: f64, // Previous close
}

impl Stocks{
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self{
        Stocks{

        }
    }

    pub fn configure_fonts(ctx: &Context){
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert("RedditSans".to_owned(),
        egui::FontData::from_static(include_bytes!("../RedditSansCondensed-VariableFont_wght.ttf")),
         );

        font_def
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "RedditSans".to_owned());
        font_def
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "RedditSans".to_owned());
        ctx.set_fonts(font_def);
    }

    pub fn render_stock_cards(&self, ui: &mut eframe::egui::Ui){
        for a in &self.stocks{
            ui.add_space(PADDING);
            let stock_name = format!("{}", a.symbol);
            if ui.visuals().dark_mode {
                ui.colored_label(WHITE, stock_name);
                ui.style_mut().visuals.hyperlink_color = WHITE;
            } else {
                ui.colored_label(BLACK, stock_name);
                ui.style_mut().visuals.hyperlink_color = BLACK;
            }
            ui.add_space(PADDING);
            let desc = Label::new(format!("Current Price: {} ", a.c));
            let desc = Label::new(format!("Change: {} ", a.d));
            let desc = Label::new(format!("Percentage Change: {} ", a.dp));
            let desc = Label::new(format!("Day High: {} ", a.h));
            let desc = Label::new(format!("Day Low: {} ", a.l));
        }
    }

    fn render_config(&mut self, ctx: &Context){
        egui::Window::new("Token").show(ctx, |ui| {
            ui.label("Enter Finnhub Token");
            let text_input = ui.text_edit_singleline(&mut self.config.token);
            if text_input.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)){
                if let Err(e) = confy::store("token", "token", API_Config{
                    token: self.config.token.to_string(),
                    token_initialize: false
                }) {
                    tracing::error!("Failed saving app state: {}", e);
                }
                self.config.token_initialize = true;
                tracing::error!("api key set");
            };
            tracing::error!(
                "{}", &self.config.token
            );
        });
    }
}

impl eframe::App for Stocks{

}