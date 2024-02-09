use std::cell::RefCell;
use ic_cdk_timers::TimerId;

thread_local! {
    pub static TIMER_STATE: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
}