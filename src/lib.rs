#[cfg(feature="builder")]
#[macro_use]
extern crate derive_builder;

pub mod models;
use models::*;
use models::common::MetaData;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Display;

pub trait QBItem
where Self: Serialize + Default + Clone + PartialEq + Sized + DeserializeOwned
{
    fn id(&self) -> Option<String>;
    fn sync_token(&self) -> Option<String>;
    fn meta_data(&self) -> Option<MetaData>;
    fn name() -> &'static str;
    fn qb_id() -> String;
}

macro_rules! impl_qb_data {
    ($($x:ident),+) => {
        $(
            impl QBItem for $x {
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
                fn name() -> &'static str {
                    stringify!($x)
                }

                #[inline]
                fn qb_id() -> String {
                    Self::name().to_lowercase()
                }
            }

            impl Display for $x {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} : {}", Self::name(), serde_json::to_string_pretty(self).unwrap())
                }
            }
        )+
   }
}

impl_qb_data!(Invoice, Vendor, Payment, Item, Estimate, Employee, Customer, CompanyInfo, Bill, Attachable, Account);