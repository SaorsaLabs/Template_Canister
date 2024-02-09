use crate::core::utils::{canister_call, log};
use super::types::{EstimateWithdrawalFeeArg, EstimateWithdrawalFeeRet};

// Impl for calling the ckBTC Minter - method estimate_withdrawal_fee
pub async fn call_ckbtc_minter() -> String {
    // Argument to be passed to the ckbtc minter canister
    // A quick way of getting these structs is to use the internet computer dashboard's generated interface
    // eg for ckbtc minter - https://dashboard.internetcomputer.org/canister/mqygn-kiaaa-aaaar-qaadq-cai
    // go to the bottom of the page and click 'Rust' in the Canister Interface section
    // these structs aren't always 100% accurate but are a good place to start :) 
    let input_arg: EstimateWithdrawalFeeArg = EstimateWithdrawalFeeArg{ amount: None};

    // This is where the call is made. Input arguments are
    // 1. The canister to call as a Str
    // 2. The method to call as a Str
    // 3. The input argument. For canister which don't need an input argument you can use () here. 
    // 4. The amount of cycles to send with the call. Eg Some(500_000) or None. Allmost all calls will be None. 
    let call_result:Result<(EstimateWithdrawalFeeRet,), (ic_cdk::api::call::RejectionCode, String)> = 
    canister_call("mqygn-kiaaa-aaaar-qaadq-cai", "estimate_withdrawal_fee", input_arg, None).await;

    // handle the result of the call
    match call_result {
        Ok(v) => {
            // If the call has completed without error the return value can be accessed here.
            // None the response is a tuple! v.0 
            let return_value: EstimateWithdrawalFeeRet = v.0;
            // for simplicity we will turn this into a string and return the result. 
            return format!("Call Result :: {:?}", return_value);
        },
        Err(e) => {
            // If the call throws an error then the error can be accessed here. 
            // Note inter-canister call errors are a tuple e.0 and e.1; 
            // these can be added to the canister log as follows
            log(format!("Canister has thrown an error - {:?}, {}", e.0, e.1));
            // Return error as string
            return format!("Canister has thrown an error - {:?}, {}", e.0, e.1);
        }
    }
}