mod elements;
mod files;
mod initialize;
mod users;
pub use files::*;
pub use users::*;
mod utils;

use crate::files::types::MyStrings;
use ic_cdk_macros::*;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::{
    s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade,
};
use initialize::*;

pub mod custom_traits;

// before upgrading the code of the canister (before deploying the app)
#[pre_upgrade]
fn pre_upgrade() {
    stable_memory_pre_upgrade();
}

// after upgrading the code of the canister
#[post_upgrade]
fn post_upgrade() {
    stable_memory_post_upgrade(0);
}
