use client::Client;
use failure::Error;
use project::ProjectRef;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub slug: String,
    pub id: u64,
    pub name: String,
    pub scm_id: String,
    pub state: String,
    pub status_message: String,
    pub forkable: bool,
    pub project: ProjectRef,
    pub public: bool,
}

impl fmt::Display for Repository {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.project.key, self.name)
    }
}

impl Client {
    pub fn repositories(&self, project_key: &str) -> Result<Vec<Repository>, Error> {
        let url = format!("rest/api/1.0/projects/{}/repos", project_key);

        self.get_paged(&url)
    }

    pub fn repository(
        &self,
        project_key: &str,
        repository_slug: &str,
    ) -> Result<Repository, Error> {
        let url = format!(
            "rest/api/1.0/projects/{}/repos/{}",
            project_key, repository_slug
        );

        self.get(&url)
    }

    pub fn repository_files(
        &self,
        project_key: &str,
        repository_slug: &str,
    ) -> Result<Vec<PathBuf>, Error> {
        let url = format!(
            "rest/api/1.0/projects/{}/repos/{}/files",
            project_key, repository_slug
        );

        Ok(self
            .get_paged::<String>(&url)?
            .into_iter()
            .map(PathBuf::from)
            .collect())
    }

    pub fn repository_file_contents(
        &self,
        project_key: &str,
        repository_slug: &str,
        path: &str,
    ) -> Result<String, Error> {
        let url = format!(
            "rest/api/1.0/projects/{}/repos/{}/browse/{}",
            project_key, repository_slug, path
        );

        Ok(self.get_lines_paged(&url)?.join("\n"))
    }
}

impl Repository {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }
}
