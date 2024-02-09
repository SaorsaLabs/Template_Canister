use ic_cdk_macros::{query, update};
use crate::core::{runtime::RUNTIME_STATE, utils::log};
use super::{btree_logic::{add_to_btree, remove_from_btree}, logic::call_ckbtc_minter};

// writing text/ data to the canister logs
#[update]
fn write_to_canister_log(input_text: String){
    log("[][] -- Write to Canister Method Called -- [][]");
    // use format to add more complex data
    let num: u64 = 101;
    log(format!("Log with some data - {}. Input Text :: {}", num, input_text));
}

// on init of canister it saves it's own canister ID this can be queried
#[query]
fn get_canister_own_id() -> String {
    let id: String = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_self_id()
    });
    return id;
}

// add data to stable memory BTreeMap using function defined in btree_logic.rs
#[update]
fn add_btree_method(name: String, nickname: String, age: u64){
    add_to_btree(name, nickname, age);
}

// remove value from stable memory BTreeMap using function defined in btree_logic.rs
#[update]
fn remove_btree_method(name: String){
    remove_from_btree(name);
}

// Make a call to another canister
// This method MUST be an update or composite-query method to work. 
#[update]
async fn make_call_to_ckbtc_minter() -> String {
    // we're going to gate this method so it can only be called by authorised callers
    RUNTIME_STATE.with(|s|{
        s.borrow().data.check_authorised(ic_cdk::caller().to_text())
    });
    // call the function defined in logic.rs 
    let icc_result: String = call_ckbtc_minter().await;

    // update the canister metrics to count the call 
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.metrics.increment_total_api() // note the borrow_mut() not borrow() used in the admin check.
    });

    // return the call result
    return icc_result;
}

#[update]
fn clear_all_logs(){
    // we're going to gate this method so it can only be called by admins
    RUNTIME_STATE.with(|s|{
        s.borrow().data.check_admin(ic_cdk::caller().to_text())
    });
    // clear canister logs
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.clear_logs()
    });
}










