use super::*;
use eframe::egui;
use reqwest::StatusCode;
use serde_json::json;

impl HooklineApp {
    pub fn display_main_panel(&mut self) -> Vec<Box<dyn FnOnce(&mut egui::Ui, &mut HooklineApp)>> {
        let mut a: Vec<Box<dyn FnOnce(&mut egui::Ui, &mut HooklineApp)>> = vec!();

        match &self.activity {
            HooklineActivity::LoggedOut => {
                a.push(Box::new(|ui: &mut egui::Ui, app: &mut HooklineApp| {
                    ui.heading("Sign In To Phishin!");
                }));
                a.push(Box::new(|ui: &mut egui::Ui, app: &mut HooklineApp| {
                    ui.label("E-mail:");
                    ui.text_edit_singleline(&mut app.vars.cred_user);
                    ui.label("Password:");
                    ui.text_edit_singleline(&mut app.vars.cred_pass);
                }));
                a.push(Box::new(|ui: &mut egui::Ui, app: &mut HooklineApp| {
                    ui.label(egui::RichText::new(&app.vars.last_cred_err).color(egui::Color32::RED));

                    if ui.button("Log In").clicked() {
                        let login = app.phishin_api_req("/auth/login", json!({
                            "email": &app.vars.cred_user.as_str(),
                            "password": &app.vars.cred_pass.as_str(),
                        }));

                        match login.status() {
                            StatusCode::UNAUTHORIZED => {
                                app.vars.last_cred_err = String::from("The email or password is incorrect.");
                            },

                            StatusCode::OK => {
                                let acc_token = login.json::<SuccessfulLogin>().unwrap();
                                app.activity = HooklineActivity::Player(PhishinAccount::Acc(acc_token), PlayerActivity::Browsing(BrowsePage::ByYears));
                            },

                            _ => {

                            }
                        }
                    }

                    if ui.button("Listen As A Guest").clicked() {
                        app.activity = HooklineActivity::Player(PhishinAccount::Guest, PlayerActivity::Browsing(BrowsePage::ByYears));
                    }
                }));
            },

            HooklineActivity::Player(acc, p) => {
                match p {
                    PlayerActivity::Browsing(b) => {
                        match b {
                            BrowsePage::ByYears => {
                                a.push(Box::new(|ui: &mut egui::Ui, app: &mut HooklineApp| {
                                    match &app.year_list {
                                        Some(yl) => {
                                            for year in yl.iter().rev() {
                                                ui.heading(year.period.clone());
                                            }
                                        },
                                        None => {
                                            app.year_list = Some(app.phishin_api_req("/years", serde_json::Value::Null).json::<Vec<Year>>().unwrap());
                                        }
                                    }
                                }));
                            },

                            /*BrowsePage::InYearRange(yr) => {

                            },

                            BrowsePage::Show(show) => {

                            }*/
                        }
                    }
                }
            }
        }

        a
    }
}
