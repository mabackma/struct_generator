#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifiers {
    #[serde(flatten)]
    pub tree_identifiers: TreeIdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    #[serde(flatten)]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNumber {
    #[serde(flatten)]
    pub tree_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeClass {
    #[serde(flatten)]
    pub tree_class: CoTreeClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifier {
    #[serde(flatten)]
    pub tree_identifier: TreeIdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    #[serde(flatten)]
    pub r#type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeListItem {
    #[serde(flatten)]
    pub tree_list_item: TreeListItemType,
}

