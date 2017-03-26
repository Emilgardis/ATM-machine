use uuid::Uuid;
use chrono;
use interface::schemas::owners;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, AsChangeset)]
pub struct Owner {
    /// An end-user
    id: Uuid, // Id of owner.
    pub name: String,
    pub registered: chrono::DateTime<chrono::UTC>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub date_of_birth: Option<chrono::DateTime<chrono::UTC>>,
}

impl Owner {
    pub fn new<'a>(name: &'a str) -> NewOwner<'a> {
        NewOwner {
            id: Uuid::new_v4(),
            name: name,
            registered: chrono::UTC::now(),
            email: None,
            phone_number: None,
            date_of_birth: None,
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Insertable)]
#[table_name="owners"]
pub struct NewOwner<'a> {
    id: Uuid,
    pub name: &'a str,
    pub registered: chrono::DateTime<chrono::UTC>,
    pub email: Option<&'a str>,
    pub phone_number: Option<&'a str>,
    // Stored as datetime but should be date, this is because of lacking impls
    // in diesel
    date_of_birth: Option<chrono::DateTime<chrono::UTC>>,
}


impl<'a> NewOwner<'a> {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn set_email<S>(mut self, email: S) -> NewOwner<'a>
    where S: Into<Option<&'a str>>{
        // Need to validate somewhere.
        self.email = email.into();
        self
    }
    pub fn set_phone_number<S>(mut self, phone_number: S) -> NewOwner<'a>
    where S: Into<Option<&'a str>>{
        self.phone_number = phone_number.into();
        self
    }
    pub fn set_date_of_birth<S>(mut self, date: S) -> NewOwner<'a>
    where S: Into<Option<chrono::DateTime<chrono::UTC>>>{
        self.date_of_birth = date.into();
        self
    }

}
