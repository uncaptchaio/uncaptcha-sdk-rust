use bon::Builder;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Map;

pub trait Task {
    type Solution: DeserializeOwned;
    fn get_name(&self) -> &'static str;
}

#[derive(Builder, Serialize)]
pub struct TurnstileTask {
    /// URL where the captcha was found, must have a path or at least a leading '/'
    url: String,
    /// Captcha's sitekey - always starts with 0x
    sitekey: String,
    // action is an optional Turnstile field
    action: Option<String>,
    /// cdata is an optional Turnstile field
    cdata: Option<String>,
    /// Optional - preferred for stability
    /// Format: [http/socks5]://<user:pass@>ip:port
    proxy: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TurnstileSolution {
    pub token: String,
}

#[derive(Builder, Serialize)]
pub struct WAFTask {
    /// URL where the captcha has been triggered
    url: String,
    /// Proxy that received the captcha, IP must match
    /// Format: [http/socks5]://<user:pass@>ip:port
    proxy: String,
    /// User agent that did the initial request
    /// Supports Chrome/Firefox/Safari on Windows/Linux/macOS/Android/iOS
    user_agent: Option<String>,
    /// Challenge HTML -- 'Cf-Mitigated' header must be present and set to 'challenge' to proceed.
    /// If this is not provided, API will automatically fetch.
    /// It is recommended to provide it by yourself for stability reasons (e.g. API could not get any challenge due to TLS differences).
    html: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WAFSolution {
    pub clearance: String,
    pub cf_bm: Option<String>,
    pub cf_rt: String,
    pub attributes: Map<String, serde_json::Value>,
    pub headers: Map<String, serde_json::Value>,
}

impl Task for TurnstileTask {
    type Solution = TurnstileSolution;
    fn get_name(&self) -> &'static str {
        "turnstile"
    }
}

impl Task for WAFTask {
    type Solution = WAFSolution;
    fn get_name(&self) -> &'static str {
        "waf"
    }
}
