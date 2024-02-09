use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs }; // for outbound
use ic_cdk_macros::{update, query};
use crate::core::runtime::RUNTIME_STATE;

use super::{
    outbound::transform_impl, 
    inbound::{http_request_impl, HttpRequestInbound, HttpResponseInbound}
};


// #[update]
// async fn test_http_outcall() -> f32 {
//     test_outcall().await
// }

// required to process response from outbound http call
#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    transform_impl(raw)
}

#[query]
fn http_request(req: HttpRequestInbound) -> HttpResponseInbound {
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.metrics.increment_http_outcalls()
    });
    http_request_impl(req)
}

#[update]
fn http_request_update(req: HttpRequestInbound) -> HttpResponseInbound {
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.metrics.increment_http_outcalls()
    });
    http_request_impl(req)
}