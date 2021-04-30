use std::collections::HashMap;

pub struct WalletInfo {
    pub coins: HashMap<String, u32>
}

pub trait ApiFeed {
    fn system_status(&self) -> bool;
    fn wallet_info(&self) -> WalletInfo;
}