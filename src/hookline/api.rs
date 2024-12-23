use super::SuccessfulLogin;
pub struct PhishinAPIRequest {
    pub reqtype: reqwest::Method,
    pub url: String,
    pub body: Option<serde_json::Value>,
    pub auth: Option<String>,
}

impl PhishinAPIRequest {
    pub fn demand(t: reqwest::Method, u: impl Into<String>) -> PhishinAPIRequest {
        PhishinAPIRequest {
            reqtype: t,
            url: u.into(),
            body: None,
            auth: None,
        }
    }

    pub fn with_body(mut self, body: serde_json::Value) -> PhishinAPIRequest {
        self.body = Some(body);
        self
    }

    pub fn as_user(mut self, user: SuccessfulLogin) -> PhishinAPIRequest {
        self.auth = Some(user.jwt);
        self
    }
}
