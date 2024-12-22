use core::f32;
use std::{thread::sleep, time::Duration};

use authentic::credential::JsonWebTokenCredential;
use eframe::egui::{Color32, Painter, Pos2, Stroke};
use reqwest::blocking::*;
use serde_json::Value;

pub mod ui;

pub struct HooklineApp {
    pub activity: HooklineActivity,
    pub client: reqwest::blocking::Client,
    pub vars: Vars,
    circles: Vec<BackgroundDonut>
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

    size: f32
}

impl BackgroundDonut {
    fn velocity_lerp(&mut self) {
        let lerp = 0.5 - 0.5 * f32::cos((f32::consts::PI / 2.0) * (self.t as f32 / self.targ_t as f32));
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

    fn glide(&mut self) {
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;
    }

    fn random() -> BackgroundDonut {
        BackgroundDonut {
            x: (600.0 * rand::random::<f32>() + 20.0),
            y: (600.0 * rand::random::<f32>() + 20.0),
            vx: -2.0 + 4.0 * rand::random::<f32>(),
            vy: -2.0 + 4.0 * rand::random::<f32>(),
            begin_vx: -2.0 + 4.0 * rand::random::<f32>(),
            begin_vy: -2.0 + 4.0 * rand::random::<f32>(),
            targ_vx: -2.0 + 4.0 * rand::random::<f32>(),
            targ_vy: -2.0 + 4.0 * rand::random::<f32>(),
            t: 0,
            targ_t: 120,
            size: 20.0
        }
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
            donut.glide();
            donut.velocity_lerp();
        }
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
                BackgroundDonut::random()
            )
        }
    }
}

pub enum HooklineActivity {
    LoggedOut,
    Browsing(PhishinAccount)
}

pub enum PhishinAccount {
    Guest,
    Acc(JsonWebTokenCredential),
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
