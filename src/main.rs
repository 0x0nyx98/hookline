use eframe::egui::{self, Color32, LayerId, Layout, Painter, Rect, Style, UiBuilder};

mod hookline;
use hookline::{HooklineApp, HooklineActivity};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Hookline",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<HooklineApp>::default())
        }),
    )
}

impl eframe::App for HooklineApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let draw = ctx.layer_painter(LayerId::background());

        self.donuts_bg(&draw);

        let clear_frame = egui::containers::Frame::dark_canvas(&Style::default()).fill(Color32::TRANSPARENT);
        let dark_frame = egui::containers::Frame::dark_canvas(&Style::default()).fill(Color32::from_rgba_unmultiplied(15, 15, 20, 180));

        match self.activity {
            HooklineActivity::Player(_, _) => {
                egui::TopBottomPanel::bottom("song-dash").frame(dark_frame).exact_height(200.0).show(ctx, |ui| {

                });

                egui::SidePanel::left("pages").frame(dark_frame).exact_width(80.0).show(ctx, |ui| {

                });
            },
            _ => {}
        }

        self.bg_panel(&draw, ctx.available_rect());

        egui::CentralPanel::default().frame(clear_frame).show(ctx, |ui| {
            let fullrect = ctx.available_rect().shrink(80.0);

            let n = self.display_main_panel().len();

            let mut rects = vec!();

            if n == 1 {
                rects.push(fullrect);
            }

            let mut splitting_rect = fullrect;

            for i in 1..=(n-1) {
                let split = splitting_rect.split_top_bottom_at_fraction(1f32 / (n - i + 1) as f32);
                rects.push(split.0);
                splitting_rect = split.1;

                if i == n - 1 {
                    rects.push(split.1);
                }
            }

            let mut k = 0;

            for portion in self.display_main_panel() {
                let u = UiBuilder::new().max_rect(rects[k]).layout(Layout::top_down(egui::Align::Center));
                k += 1;

                ui.allocate_new_ui(u, |ui| {
                    portion(ui, self);
                });
            }
        });

        ctx.request_repaint();
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}