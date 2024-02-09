use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
pub struct WorkingStats {
    timer_active: bool,
    pub last_update_time: u64,
    pub metrics: Metrics,
}

impl WorkingStats {
    pub fn update_timer(&mut self, state: bool) -> String {
        self.timer_active = state;
        return "timer_active has been updated".to_string();
    }

    pub fn get_timer_state(&self) -> bool {
        return self.timer_active.clone();
    }

    pub fn get_metrics(&self) -> Metrics {
        return self.metrics.clone();
    }
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
pub struct Metrics {
    total_errors: u64,
    total_api_requests: u64,
}

impl Metrics {
    pub fn increment_total_errors(&mut self){
        self.total_errors += 1;
    }

    pub fn increment_total_api(&mut self){
        self.total_api_requests += 1;
    }
}