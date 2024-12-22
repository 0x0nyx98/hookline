use eframe::egui::{self, Layout, Rect, UiBuilder};

mod hookline;
use hookline::HooklineApp;

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
        egui::CentralPanel::default().show(ctx, |ui| {
            let fullrect = ui.max_rect();
            let fullrect = fullrect.split_left_right_at_fraction(0.1).1;
            let fullrect = fullrect.split_left_right_at_fraction(0.9).0; // giving tje ui a hair cut :3
            let fullrect = fullrect.split_top_bottom_at_fraction(0.1).1;
            let fullrect = fullrect.split_top_bottom_at_fraction(0.9).0;

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
    }
}