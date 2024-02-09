use candid::{define_function, CandidType, Deserialize};
use serde_bytes::ByteBuf;
pub type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Token {}

define_function!(pub StreamingCallbackFunction : (Token) -> (StreamingCallbackHttpResponse) query);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum StreamingStrategy {
    Callback {
        callback: StreamingCallbackFunction,
        token: Token,
    },
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StreamingCallbackHttpResponse {
    pub body: ByteBuf,
    pub token: Option<Token>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequestInbound {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: ByteBuf,
    pub certificate_version: Option<u16>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponseInbound {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
    pub upgrade: Option<bool>,
    pub streaming_strategy: Option<StreamingStrategy>,
}

pub fn http_request_impl(req: HttpRequestInbound) -> HttpResponseInbound {
    let parts: Vec<&str> = req.url.split('?').collect();
    match parts[0] {
        // redirect
        "/data" => HttpResponseInbound {
            status_code: 301,
            headers: vec![(
                "location".to_string(),
                "https://221Bravo.App/".to_string(),
            )],
            body: ByteBuf::new(),
            upgrade: Some(true),
            streaming_strategy: None,
        },
        // Test route /ok
        "/ok" => { test_return_ok() },
        // Test route /err
        "/err" => { test_return_fail() },
        _ => { test_return_fail() }
    }
}

fn test_return_ok() -> HttpResponseInbound {
    let content_body = "HELLO WORLD!";
    let mut headers = vec![
        (
            "Content-Type".to_string(),
            "text/plain; version=0.0.4".to_string(),
        ),
        ("Content-Length".to_string(), content_body.len().to_string()),
    ];
    headers.append(&mut security_headers());
    let ret: HttpResponseInbound = HttpResponseInbound {
        status_code: 200,
        headers,
        body: ByteBuf::from(content_body),
        upgrade: None,
        streaming_strategy: None,
    };
    return ret;
}

fn test_return_fail() -> HttpResponseInbound {
    let ret = HttpResponseInbound {
            status_code: 500,
            headers: security_headers(),
            body: ByteBuf::from(format!("DOH!")),
            upgrade: None,
            streaming_strategy: None,
        };
    return ret;
}

/// List of recommended security headers as per https://owasp.org/www-project-secure-headers/
/// These headers enable browser security features (like limit access to platform apis and set
/// iFrame policies, etc.).
pub fn security_headers() -> Vec<HeaderField> {
    vec![
        ("X-Frame-Options".to_string(), "DENY".to_string()),
        ("X-Content-Type-Options".to_string(), "nosniff".to_string()),
        (
            "Strict-Transport-Security".to_string(),
            "max-age=31536000 ; includeSubDomains".to_string(),
        ),
        // "Referrer-Policy: no-referrer" would be more strict, but breaks local dev deployment
        // same-origin is still ok from a security perspective
        ("Referrer-Policy".to_string(), "same-origin".to_string()),
        (
            "Permissions-Policy".to_string(),
            "accelerometer=(),\
             ambient-light-sensor=(),\
             autoplay=(),\
             battery=(),\
             camera=(),\
             clipboard-read=(),\
             clipboard-write=(self),\
             conversion-measurement=(),\
             cross-origin-isolated=(),\
             display-capture=(),\
             document-domain=(),\
             encrypted-media=(),\
             execution-while-not-rendered=(),\
             execution-while-out-of-viewport=(),\
             focus-without-user-activation=(),\
             fullscreen=(),\
             gamepad=(),\
             geolocation=(),\
             gyroscope=(),\
             hid=(),\
             idle-detection=(),\
             interest-cohort=(),\
             keyboard-map=(),\
             magnetometer=(),\
             microphone=(),\
             midi=(),\
             navigation-override=(),\
             payment=(),\
             picture-in-picture=(),\
             publickey-credentials-get=(self),\
             screen-wake-lock=(),\
             serial=(),\
             speaker-selection=(),\
             sync-script=(),\
             sync-xhr=(self),\
             trust-token-redemption=(),\
             usb=(),\
             vertical-scroll=(),\
             web-share=(),\
             window-placement=(),\
             xr-spatial-tracking=()"
                .to_string(),
        ),
    ]
}