use ic_cdk_macros::{init, pre_upgrade, post_upgrade};
use ic_stable_structures::Memory;
use ic_stable_structures::writer::Writer;

use crate::core::runtime::RuntimeState;
use super::runtime::{Data, RUNTIME_STATE};
use super::stable_memory::get_upgrades_memory;
use super::utils::log;
use super::working_stats::WorkingStats;

#[init]
fn init(admin: String) {
    let mut data = Data::default();
    let stats = WorkingStats::default();
    data.add_admin(admin.clone());
    data.add_authorised(admin);
    data.set_self_id(ic_cdk::api::id());
    let runtime_state = RuntimeState { data, stats };
    RUNTIME_STATE.with(|state| *state.borrow_mut() = runtime_state);
    log("Canister Initialised");
}

#[pre_upgrade]
fn pre_upgrade() {
    // Serialize the state.
    // This example is using CBOR, but you can use any data format you like.
    let mut state_bytes = vec![];
    RUNTIME_STATE.with(|s| ciborium::ser::into_writer(&*s.borrow(), &mut state_bytes))
        .expect("failed to encode state");

    // Write the length of the serialized bytes to memory, followed by the
    // by the bytes themselves.
    let len = state_bytes.len() as u32;
    let mut memory = get_upgrades_memory();
    let mut writer = Writer::new(&mut memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap()
}

#[post_upgrade]
fn post_upgrade() {
    let ug_memory = get_upgrades_memory();

    // Read the length of the state bytes.
    let mut state_len_bytes = [0; 4];
    ug_memory.read(0, &mut state_len_bytes);
    let state_len = u32::from_le_bytes(state_len_bytes) as usize;

    // Read the bytes
    let mut state_bytes = vec![0; state_len];
    ug_memory.read(4, &mut state_bytes);

    // Deserialize and set the state.
    let state = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
    RUNTIME_STATE.with(|s| {
        *s.borrow_mut() = state
    });
}

