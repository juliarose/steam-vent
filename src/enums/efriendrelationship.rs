use num_enum::{IntoPrimitive, TryFromPrimitive};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum EFriendRelationship {
    None = 0,
    Blocked = 1,
    RequestRecipient = 2,
    Friend = 3,
    RequestInitiator = 4,
    Ignored = 5,
    IgnoredFriend = 6,
    SuggestedFriend = 7,
}
