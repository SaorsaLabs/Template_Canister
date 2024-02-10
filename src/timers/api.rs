use ic_cdk_macros::update;
use ic_cdk_timers::TimerId;
use crate::{
    core::{runtime::RUNTIME_STATE, utils::log}
    };
use super::state::TIMER_STATE;
use super::timers::start_processing_timer;

// [][] -- TIMER METHODS -- [][]
#[update]
fn stop_all_timers() -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});

    // clear timers
    TIMER_STATE.with(|timer_ids| {
        let vec1: &mut std::cell::RefMut<Vec<TimerId>> = &mut timer_ids.borrow_mut();
        for i in vec1.iter() {
            ic_cdk_timers::clear_timer(*i);
        }
        vec1.clear();
    });

    // update working stats
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.update_timer(false)
    });   

    log("[][] ---- All timers stopped ---- [][]");
    return String::from("All timers stopped");
}

#[update]
fn start_test_timer(secs: u64) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});

    let ret;
    // check if running already
    let is_running = RUNTIME_STATE.with(|s|{
        s.borrow().stats.get_timer_state()
    });
     if is_running == true {
        ret = String::from("Main quotes timer is already running");
    } else {
        start_processing_timer(secs);
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().stats.update_timer(true)
        });
        ret = String::from("Test Timer has been started");
        log("[][] ---- Starting Test Timer ---- [][]");
    }
    return ret;
}

