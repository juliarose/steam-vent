pub mod app;
#[cfg(feature = "tf2")]
pub mod tf2;

use protobuf::Message;
use std::fmt::Debug;

pub trait GCMessage: Debug + Message {}