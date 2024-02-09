use candid::CandidType;
use serde::Deserialize;

// Struct for data returned from ckBTC Minter call
#[derive(CandidType, Deserialize, Debug)]
pub struct EstimateWithdrawalFeeRet {
  pub minter_fee: u64,
  pub bitcoin_fee: u64,
}

// Struct for call arguments to use in inter-canister call. 
#[derive(CandidType, Deserialize)]
pub struct EstimateWithdrawalFeeArg { pub amount: Option<u64> }
