use std::{thread::sleep, time::Duration};

use authentic::credential::JsonWebTokenCredential;
use reqwest::blocking::*;
use serde_json::Value;

pub mod ui;

pub struct HooklineApp {
    pub activity: HooklineActivity,
    pub client: reqwest::blocking::Client,
    pub vars: Vars
}

impl HooklineApp {
    fn phishin_api_req(&self, req: &str, body: Value) -> Response {
        let mut s = String::from("https://phish.in/api/v2");
        s.push_str(req);

        loop { 
            match self.client.post(&s).json(&body).send() {
                Ok(resp) => { break resp; }
                Err(e) => { println!("retrying request to phish.in (is it down?) ..."); sleep(Duration::from_secs_f32(0.5)); }
            }
        }
    }
}

impl Default for HooklineApp {
    fn default() -> Self {
        Self {
            activity: HooklineActivity::LoggedOut(),
            client: reqwest::blocking::Client::new(),
            vars: Vars::NONE
        }
    }
}

enum HooklineActivity {
    LoggedOut(),
    Browsing(PhishinAccount)
}

enum PhishinAccount {
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
