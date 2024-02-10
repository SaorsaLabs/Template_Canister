use std::borrow::Cow;
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

use crate::core::stable_memory::MAP;


#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct ExampleData {
    pub example_data1: String,
    pub example_data2: u64,
}

// Structs need to imp Storable to be used in Stable Memory
impl Storable for ExampleData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;

    // If bounded is desired
    // const BOUND: Bound = Bound::Bounded {
    //     max_size: 1_343_100,
    //     is_fixed_size: false,
    // };
}

// add to btree map
pub fn add_to_btree(key: String, value_one: String, value_two: u64){
    // MAP is defined within stable_memory.rs in core module
    // remember to update the value type on line 12 in stable_memory.rs - (this btree uses key: String, value: ExampleData).
    let value: ExampleData = ExampleData{
        example_data1: value_one,
        example_data2: value_two
    }; 
    MAP.with(|s|{
        s.borrow_mut().insert(key.clone(), value)
    });
}

// remove from btree map
pub fn remove_from_btree(key: String){
    let rm = MAP.with(|s|{
        s.borrow_mut().remove(&key)
    });
}

pub fn get_value_from_btree(key: String) -> Option<ExampleData> {
    MAP.with(|s|{
        s.borrow().get(&key)
    })
}
