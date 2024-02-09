use ic_cdk_macros::{query, update};
use super::{
    types::{LogEntry, MemoryData}, 
    runtime::RUNTIME_STATE, constants::CANISTER_VERSION
};

// [][] -- ADMIN METHODS -- [][]
#[update]
fn add_authorised(principal_id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.data.check_admin(ic_cdk::caller().to_text());
        s.data.add_authorised(principal_id)
    })
}

#[update]
fn remove_authorised(principal_id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.data.check_admin(ic_cdk::caller().to_text());
        s.data.remove_authorised(principal_id)
    })
}

#[query]
fn get_all_authorised() -> Vec<String> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
        s.data.get_all_authorised()
    })
}

#[query]
fn get_logs() -> Option<Vec<LogEntry>> {
    // Is authorised?
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text())
    });
    RUNTIME_STATE.with(|state|{
        state.borrow().data.get_logs()
    })
}

#[query]
#[cfg(target_arch = "wasm32")]
fn get_memory_stats() -> MemoryData {
    // Is authorised?
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text())
    });

  let wasm_page_size: u64 = 65536;
  let m: u64 = ic_cdk::api::stable::stable64_size() as u64 * wasm_page_size + core::arch::wasm32::memory_size(0) as u64 * wasm_page_size;
  let m2: u64 = core::arch::wasm32::memory_size(0) as u64 * wasm_page_size;
  let ret = MemoryData {
    memory: m,
    heap_memory: m2
  };
  return ret;
}

#[update]
fn add_admin(principal_id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.data.check_admin(ic_cdk::caller().to_text());
        s.data.add_admin(principal_id)
    })
}

#[update]
fn remove_admin(principal_id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.data.check_admin(ic_cdk::caller().to_text());
        s.data.remove_admin(principal_id)
    })
}

#[query]
fn get_all_admins() -> Vec<String> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
        s.data.get_all_admins()
    })
}


// [][] --- Not Gated --- [][]
#[query]
fn get_cycles_balance() -> u64 {
    let cycles: u64 = ic_cdk::api::canister_balance();
    return cycles;
}

#[update]
fn deposit_cycles() -> () {
    let amount = ic_cdk::api::call::msg_cycles_available128();
    if amount > 0 {
        ic_cdk::api::call::msg_cycles_accept128(amount);
    }
}

#[query]
fn get_canister_version() -> String {
    return CANISTER_VERSION.to_string();
}