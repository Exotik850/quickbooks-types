#[cfg(feature="builder")]
#[macro_use]
extern crate derive_builder;

pub mod models;
use models::*;
use models::common::MetaData;
use serde::Serialize;
use std::fmt::Display;

pub trait HasQBData 
where Self: Serialize
{
    fn id(&self) -> Option<String>;
    fn sync_token(&self) -> Option<String>;
    fn meta_data(&self) -> Option<MetaData>;
    fn name(&self) -> &str;
    fn qb_id(&self) -> String;
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

                #[inline]
                fn name(&self) -> &str {
                    stringify!($x)
                }

                #[inline]
                fn qb_id(&self) -> String {
                    self.name().to_lowercase()
                }
            }

            impl Display for $x {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} : {}", self.name(), serde_json::to_string_pretty(self).unwrap())
                }
            }
        )+
   }
}

impl_qb_data!(Invoice, Vendor, Payment, Item, Estimate, Employee, Customer, CompanyInfo, Bill, Attachable, Account);