use std::time::Duration;

use crate::core::{runtime::RUNTIME_STATE, utils::log};
use super::state::TIMER_STATE;

// push timer into timer state
pub fn start_processing_timer(secs: u64) {
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_data_processing())
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

// This is the function that will be called by the timer every X seconds
async fn schedule_data_processing(){
    // This function just adds message to the canister logs
    log("Timer has called the schedule_data_processing function");
    
    // update last update time 
    let time_now = ic_cdk::api::time();  // this gets the current time in nano seconds. 
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.last_update_time = time_now;
    });
}
