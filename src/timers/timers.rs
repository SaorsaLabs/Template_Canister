use std::time::Duration;

use crate::{
    core::runtime::RUNTIME_STATE,
};
use super::state::TIMER_STATE;

pub fn start_processing_timer(secs: u64) {
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_data_processing())
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

// Fetch Main quotes
async fn schedule_data_processing(){

    // update last update time 
    let time_now = ic_cdk::api::time();
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.last_update_time = time_now;
    });
    
}
