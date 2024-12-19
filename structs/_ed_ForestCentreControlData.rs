#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectData {
    #[serde(flatten)]
    pub control_object_data: ControlObjectDataType,
}

