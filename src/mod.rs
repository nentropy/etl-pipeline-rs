mod lib;
mod ops;
mod session_ctx;
mod main;
mod opencti;
mod user;
mod enums;
mod structs;
mod traits;
mod enums;

use crate::to_do::enums::TaskStatus;
use crate::to_do::structs::done::Done;
use crate::to_do::structs::pending::Pending;

pub mod enums;
pub mod structs;
pub mod traits;

pub enum ItemType {
    Pending(Pending),
    Done(Done)
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemType {
    match status {
        TaskStatus::DONE => ItemType::Done(Done::new(title)),
        TaskStatus::PENDING => ItemType::Pending(Pending::new(title))
    }
}