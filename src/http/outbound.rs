use candid::Nat;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use serde::{Serialize, Deserialize};
use serde_json::{self};

use crate::core::runtime::RUNTIME_STATE;

#[derive(Debug, Serialize, Deserialize)]
struct ReceivedData {
    data: PriceData,
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceData {
    amount: String,
    base: String,
    currency: String,
}

// EXAMPLE OUTCALL TO COINBASE
pub async fn get_icp_usd_rate() -> Result<f64, String> {
    let host = "api.coinbase.com";
    let url = format!(
        "https://{}/v2/prices/icp-usd/buy",
        host
    );

    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: format!("{host}:443"), //443
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "Defi Oracle".to_string(),
        },
    ];

    let context = Context { does_nothing: 0}; // legacy code... does nothing!
    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,                         //optional for request
        max_response_bytes: Some(5000_u64), //optional for request
        transform: Some(TransformContext::from_name("transform".to_string(), serde_json::to_vec(&context).unwrap())),
        headers: request_headers,
    };

    let cycles = 300_000_000_000;
    match http_request(request, cycles).await {
        Ok((response,)) => {

            // MORE INFO: 
            //See:https://docs.rs/ic-cdk/latest/ic_cdk/api/management_canister/http_request/struct.HttpResponse.html
            //if successful, `HttpResponse` has this structure:
            // pub struct HttpResponse {
            //     pub status: Nat,
            //     pub headers: Vec<HttpHeader>,
            //     pub body: Vec<u8>,
            // }

            let result: Result<ReceivedData, serde_json::Error> = serde_json::from_slice(&response.body);
            match result {
                Ok(v) => {
                    match v.data.amount.parse::<f64>() {
                        Ok(parsed_value) => {
                            return Ok(parsed_value);
                        }
                        Err(e) => {
                            RUNTIME_STATE.with(|s|{
                                s.borrow_mut().stats.metrics.increment_total_errors()
                            });
                            return Err(format!("Error fetching ICP/USD rate (fn get_icp_usd_rate): {}", e));
                        }
                    }
                },
                Err(e) => {
                    RUNTIME_STATE.with(|s|{
                        s.borrow_mut().stats.metrics.increment_total_errors()
                    });
                    return Err(format!("Error fetching ICP/USD rate (fn get_icp_usd_rate): {}", e));
                },
            }
        }
        Err((r, m)) => {
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().stats.metrics.increment_total_errors()
            });
            let message = format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
            return Err(format!("Error fetching ICP/USD rate (fn get_icp_usd_rate): {}", message));
        }
    }
}

// Strips all data that is not needed from the original response.
pub fn transform_impl(raw: TransformArgs) -> HttpResponse {

    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];
    
    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
        ..Default::default()
    };

    if res.status == Nat::from(200_u32) {
        res.body = raw.response.body;
    } else {
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().stats.metrics.increment_total_errors()
        });
        ic_cdk::api::print(format!("Received an error from coinbase: err = {:?}", raw));
    }
    return res;
}


// legacy - does nothing!
#[derive(Serialize, Deserialize)]
struct Context {
    does_nothing: u64
}

