use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs }; // for outbound
use ic_cdk_macros::{update, query};
use crate::core::utils::log;

use super::{
    inbound::{http_request_impl, HttpRequestInbound, HttpResponseInbound}, outbound::{get_icp_usd_rate, transform_impl}
};


#[update]
async fn test_http_outcall() -> f64 {
   let outcall = get_icp_usd_rate().await;
   match outcall {
    Ok(v) => {
        return v;
    },
    Err(e) => {
        log(format!("Test_http_outcall threw an error - {}", e));
        return 0f64;
    }
   }
}

// required to process response from outbound http call
// do not delete these.
#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    transform_impl(raw)
}

#[query]
fn http_request(req: HttpRequestInbound) -> HttpResponseInbound {
    http_request_impl(req)
}

#[update]
fn http_request_update(req: HttpRequestInbound) -> HttpResponseInbound {
    http_request_impl(req)
}