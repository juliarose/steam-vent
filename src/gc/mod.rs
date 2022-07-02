#[cfg(feature = "tf2")]
pub mod tf2;

use protobuf::Message;
use std::fmt::Debug;

pub trait App {
    const APPID: u32;
}

pub trait GCMessage: Debug + Message {}