#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequestsType {
    #[serde(flatten)]
    pub contact_requests_type: ContactRequests,
}

