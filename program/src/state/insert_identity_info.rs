use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct InsertIdentityInfo {
    pub details: String,
    pub instruction: u8,
    pub seed: String,
}

impl InsertIdentityInfo {
    pub fn new(details: String, instruction: u8, seed: String) -> Self {
        InsertIdentityInfo {
            details,
            instruction,
            seed,
        }
    }
}

