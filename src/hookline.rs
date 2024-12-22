use core::f32;
use std::{thread::sleep, time::Duration};

use authentic::credential::JsonWebTokenCredential;
use eframe::egui::{Color32, Painter, Pos2, Rect, Stroke};
use music::Year;
use reqwest::blocking::*;
use serde::Deserialize;
use serde_json::Value;

pub mod ui;
pub mod music;

pub struct HooklineApp {
    pub activity: HooklineActivity,
    pub client: reqwest::blocking::Client,
    pub vars: Vars,
    circles: Vec<BackgroundDonut>,
    pub year_list: Option<Vec<Year>>
}

struct BackgroundDonut {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    begin_vx: f32,
    begin_vy: f32,
    targ_vx: f32,
    targ_vy: f32,
    t: i32,
    targ_t: i32,

    base_size: f32,
    size: f32,
    life_time: i32,
    age: i32,

    seed: bool,
    balloon: f32
}

impl BackgroundDonut {
    fn velocity_lerp(&mut self) {
        let lerp = 0.5 - 0.5 * f32::cos(f32::consts::PI * (self.t as f32 / self.targ_t as f32));
        self.vx = (1.0 - lerp) * self.begin_vx + lerp * self.targ_vx;
        self.vy = (1.0 - lerp) * self.begin_vy + lerp * self.targ_vy;

        if self.t == self.targ_t {
            self.begin_vx = self.targ_vx;
            self.begin_vy = self.targ_vy;

            self.targ_vx = -2.0 + 4.0 * rand::random::<f32>();
            self.targ_vy = -2.0 + 4.0 * rand::random::<f32>();

            self.t = 0;
            self.targ_t = (120.0 * rand::random::<f32>() + 60.0) as i32;
        }

        self.t = self.t + 1;
    }

    fn glide(&mut self, bounds: Rect) {
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;
        
        if self.x < bounds.left() - 100.0 {
            self.x = bounds.right() + 100.0;
        }

        if self.x > bounds.right() + 100.0 {
            self.x = bounds.left() - 100.0;
        }

        if self.y < bounds.top() - 100.0 {
            self.y = bounds.bottom() + 100.0;
        }

        if self.y > bounds.bottom() + 100.0 {
            self.y = bounds.top() - 100.0;
        }
    }

    fn age(&mut self) {
        self.age = self.age + 1;

        let lerp = f32::cos((f32::consts::PI / 2.0) * (self.age as f32 / self.life_time as f32));

        self.size = self.base_size * lerp * self.balloon;

        self.balloon = 1.0 - ((1.0 - self.balloon) / 1.1);
    }

    fn random() -> BackgroundDonut {
        BackgroundDonut {
            x: (1600.0 * rand::random::<f32>() + 20.0),
            y: (1600.0 * rand::random::<f32>() + 20.0),
            vx: -2.0 + 4.0 * rand::random::<f32>(),
            vy: -2.0 + 4.0 * rand::random::<f32>(),
            begin_vx: -2.0 + 4.0 * rand::random::<f32>(),
            begin_vy: -2.0 + 4.0 * rand::random::<f32>(),
            targ_vx: -2.0 + 4.0 * rand::random::<f32>(),
            targ_vy: -2.0 + 4.0 * rand::random::<f32>(),
            t: 0,
            targ_t: 120,
            base_size: 15.0 + 20.0 * rand::random::<f32>(),
            size: 0.0,
            life_time: (1200.0 * rand::random::<f32>() + 600.0) as i32,
            age: 0,
            seed: true,
            balloon: 0.0
        }
    }

    fn random_in(r: Rect) -> BackgroundDonut {
        let mut d = BackgroundDonut::random();
        d.x = (r.width() * rand::random::<f32>() + r.left());
        d.y = (r.height() * rand::random::<f32>() + r.top());
        d
    }

    fn random_at(x: f32, y: f32) -> BackgroundDonut {
        let mut d = BackgroundDonut::random();
        d.x = x;
        d.y = y;
        d
    } 

    fn random_not_seed_at(x: f32, y: f32) -> BackgroundDonut {
        let mut d = BackgroundDonut::random_at(x, y);
        d.seed = false;
        d.life_time = (500.0 * rand::random::<f32>() + 200.0) as i32;
        d
    } 
}

impl HooklineApp {
    pub fn phishin_api_req(&self, req: &str, body: Value) -> Response {
        let mut s = String::from("https://phish.in/api/v2");
        s.push_str(req);

        loop { 
            match self.client.post(&s).json(&body).send() {
                Ok(resp) => { break resp; }
                Err(e) => { println!("retrying request to phish.in (is it down?) ..."); sleep(Duration::from_secs_f32(0.5)); }
            }
        }
    }

    pub fn donuts_bg(&mut self, p: &Painter) {
        let navy = Color32::from_rgb(28, 33, 58);
        let red = Color32::from_rgb(252, 27, 80);

        p.rect_filled(p.clip_rect(), 0.0, navy);

        for donut in &mut self.circles {
            p.circle_stroke(Pos2::new(donut.x, donut.y), donut.size, Stroke::new(donut.size * 0.8, red));
            donut.glide(p.clip_rect());
            donut.velocity_lerp();
            donut.age();
        }

        let L = self.circles.len();

        for j in 1..=L {
            let i = L - j;

            if self.circles[i].age == self.circles[i].life_time {
                if self.circles[i].seed {
                    let x = self.circles[i].x;
                    let y = self.circles[i].y;
                    self.circles.push(BackgroundDonut::random_at(x, y));

                    for k in 0..(5.0 * rand::random::<f32>() + 4.0) as i32 {
                        self.circles.push(BackgroundDonut::random_not_seed_at(x, y));
                    }
                }

                self.circles.remove(i);
            }
        }

        p.rect_filled(p.clip_rect().shrink(80.0), 20.0, Color32::from_rgba_unmultiplied(20, 25, 35, 180));
    }
}

impl Default for HooklineApp {
    fn default() -> Self {
        Self {
            activity: HooklineActivity::LoggedOut,
            client: reqwest::blocking::Client::new(),
            vars: Vars::NONE,
            circles: vec!(
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random(),
                BackgroundDonut::random()
            ),
            year_list: None
        }
    }
}

pub enum HooklineActivity {
    LoggedOut,
    Player(PhishinAccount, PlayerActivity)
}

pub enum PlayerActivity {
    Browsing(BrowsePage)
}

pub enum BrowsePage {
    ByYears,
    //InYearRange(YearRange),
    //Show(Show)
}

pub enum PhishinAccount {
    Guest,
    Acc(SuccessfulLogin),
}

#[derive(Deserialize)]
struct SuccessfulLogin {
    jwt: String,
    username: String,
    email: String
}

pub struct Vars {
    cred_user: String,
    cred_pass: String,
    last_cred_err: String,
}

impl Vars {
    const NONE: Vars = Vars {
        cred_user: String::new(),
        cred_pass: String::new(),
        last_cred_err: String::new(),
    };
}
