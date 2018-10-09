use client::Client;

use failure::Error;
use std::fmt;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct User {
    name: String,
    email_address: Option<String>,
    id: u64,
    display_name: String,
    active: bool,
    slug: String,
    #[serde(rename = "type")]
    _type: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

impl Client {
    pub fn users_admin(&self) -> Result<Vec<User>, Error> {
        let url = "rest/api/1.0/admin/users";
        self.get_paged(&url)
    }

    pub fn users(&self) -> Result<Vec<User>, Error> {
        let url = "rest/api/1.0/users";
        self.get_paged(&url)
    }

    pub fn user(&self, user_slug: &str) -> Result<User, Error> {
        let url = format!("rest/api/1.0/users/{}", user_slug);
        self.get(&url)
    }
}

impl User {
    pub fn name(&self) -> &str {
        &self.name
    }
}
