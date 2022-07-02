
#[derive(Debug, Clone, PartialEq)]
pub enum ItemCustomization {
    GiftedBy,
    CraftedBy,
    Decal,
    Killstreak,
    Paint,
    Festivizer,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecipeComponent {
    pub subject_item_id: u64,
    pub attribute_index: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetItemPosition {
    pub item_id: u64,
    pub position: u32,
}