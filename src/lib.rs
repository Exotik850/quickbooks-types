#[cfg(feature="builder")]
#[macro_use]
extern crate derive_builder;

pub mod models;
use models::*;
use models::common::MetaData;

pub trait HasQBData {
    fn id(&self) -> Option<String>;
    fn sync_token(&self) -> Option<String>;
    fn meta_data(&self) -> Option<MetaData>;
    fn name(&self) -> &str;
}

macro_rules! impl_qb_data {
    ($($x:ident),+) => {
        $(
            impl HasQBData for $x {
                fn id(&self) -> Option<String> {
                    self.id.clone()
                }
                
                fn sync_token(&self) -> Option<String> {
                    self.sync_token.clone()
                }
                
                fn meta_data(&self) -> Option<MetaData> {
                    self.meta_data.clone()
                }

                fn name(&self) -> &str {
                    stringify!($x)
                }
            }
        )+
   }
}

impl_qb_data!(Invoice, Vendor, Payment, Item, Estimate, Employee, Customer, CompanyInfo, Bill, Attachable, Account);