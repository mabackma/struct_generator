#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleActor {
    #[serde(flatten)]
    pub responsible_actor: ResponsibleActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActingDate {
    #[serde(flatten)]
    pub acting_date: ActingDateType,
}

