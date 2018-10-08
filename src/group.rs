use client::Client;
use failure::Error;
use std::fmt;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    name: String,
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Group {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Client {
    pub fn groups_admin(&self) -> Result<Vec<Group>, Error> {
        let url = "rest/api/1.0/admin/groups";

        self.get_paged(&url)
    }

    pub fn groups(&self) -> Result<Vec<String>, Error> {
        let url = "rest/api/1.0/groups";

        self.get_paged(&url)
    }
}
