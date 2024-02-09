use candid::CandidType;
use serde::{Serialize, Deserialize};

// [][] --- Types for Utils --- [][]
#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct MemoryData {
   pub memory: u64,
   pub heap_memory: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub text: String,
}