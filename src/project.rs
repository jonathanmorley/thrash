use client::Client;
use failure::Error;
use std::fmt;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    key: String,
    id: u64,
    name: String,
    description: Option<String>,
    public: bool,
    #[serde(rename = "type")]
    _type: String,
    //repositories: Vec<Repository>,
    //auth: Authorization,
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.key)
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRef {
    pub key: String,
}

impl Client {
    pub fn projects(&mut self) -> Result<Vec<Project>, Error> {
        let url = "rest/api/1.0/projects";

        self.get_paged(&url)
    }

    pub fn project(&mut self, project_key: &str) -> Result<Project, Error> {
        let url = format!("rest/api/1.0/projects/{}", project_key);

        self.get(&url)
    }
}

impl Project {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}
