use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable)]
pub struct Owner {
    /// An end-user
    #[serde(rename="OwnerId")]
    pub id: Uuid, // Id of owner.
    #[serde(rename="Name")]
    pub name: String,
}

impl Owner {
    pub fn new<T: AsRef<str>>(name: T) -> Owner {
        Owner {
            id: Uuid::new_v4(),
            name: name.as_ref().into(),
        }
    }
}
