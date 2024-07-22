use borsh::{BorshDeserialize, BorshSerialize};

pub struct AddInfo {
    pub info: String
}

impl AddInfo{
    pub fn new (info:String)-> Self {

        AddInfo{
            info,
        }
        
    }
}