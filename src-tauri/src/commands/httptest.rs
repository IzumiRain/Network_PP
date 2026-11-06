use serde::Serialize;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HttpTestResult {
    pub url: String,
    pub method: String,
    pub status: u16,
    pub status_text: String,
    pub ok: bool,
    pub total_ms: u64,
    pub body_bytes: usize,
    pub headers: Vec<HttpHeader>,
    pub server: Option<String>,
    pub content_type: Option<String>,
    pub error: Option<String>,
}

fn classify(status: u16) -> &'static str {
    match status {
        100..=199 => "Informational",
        200..=299 => "Success",
        300..=399 => "Redirect",
        400..=499 => "Client error",
        500..=599 => "Server error",
        _ => "Unknown",
    }
}

fn build_agent(timeout_ms: u64, proxy: Option<&str>) -> Result<ureq::Agent, String> {
    let mut builder = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_millis(timeout_ms.max(500)))
        .timeout(Duration::from_millis(timeout_ms.max(500)));
    if let Some(p) = proxy {
        if !p.trim().is_empty() {
            let px =
                ureq::Proxy::new(p.trim()).map_err(|_| format!("invalid proxy '{p}'"))?;
            builder = builder.proxy(px);
        }
    }
    Ok(builder.build())
}

pub fn http_test(url: &str, method: &str, timeout_ms: u64, proxy: Option<&str>) -> HttpTestResult {
    let method = method.to_uppercase();
    let mut result = HttpTestResult {
        url: url.to_string(),
        method: method.clone(),
        status: 0,
        status_text: "—".into(),
        ok: false,
        total_ms: 0,
        body_bytes: 0,
        headers: Vec::new(),
        server: None,
        content_type: None,
        error: None,
    };

    let t0 = Instant::now();
    let run = || -> Result<ureq::Response, String> {
        let agent = build_agent(timeout_ms, proxy)?;
        let outcome = match method.as_str() {
            "HEAD" => agent.head(url).call(),
            _ => agent.get(url).call(),
        };
        // A non-2xx status is still a completed HTTP exchange — ureq hands us
        // the response inside the error; unwrap it instead of failing.
        match outcome {
            Ok(resp) => Ok(resp),
            Err(ureq::Error::Status(_, resp)) => Ok(resp),
            Err(e) => Err(e.to_string()),
        }
    };

    match run() {
        Ok(resp) => {
            result.status = resp.status();
            result.status_text = format!("{} {}", resp.status(), classify(resp.status()));
            result.ok = (200..300).contains(&resp.status());
            for name in resp.headers_names() {
                let value = resp.header(&name).unwrap_or("").to_string();
                match name.as_str() {
                    "server" => result.server = Some(value.clone()),
                    "content-type" => result.content_type = Some(value.clone()),
                    _ => {}
                }
                result.headers.push(HttpHeader { name, value });
            }
            result.body_bytes = resp
                .header("content-length")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
        }
        Err(e) => result.error = Some(e),
    }
    result.total_ms = t0.elapsed().as_millis() as u64;
    result
}
