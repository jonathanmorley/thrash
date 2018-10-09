use client::Client;
use failure::Error;
use serde::{de, Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PermissionStatus {
    pub permitted: bool,
}

#[derive(Debug, Serialize, PartialEq, Eq, Hash)]
pub enum ProjectPermission {
    Read,
    Write,
    Admin,
}

impl FromStr for ProjectPermission {
    type Err = Error;

    fn from_str(s: &str) -> Result<ProjectPermission, Error> {
        match s {
            "PROJECT_READ" => Ok(ProjectPermission::Read),
            "PROJECT_WRITE" => Ok(ProjectPermission::Write),
            "PROJECT_ADMIN" => Ok(ProjectPermission::Admin),
            _ => bail!("unexpected permission {}", s),
        }
    }
}

impl fmt::Display for ProjectPermission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProjectPermission::Read => write!(f, "PROJECT_READ"),
            ProjectPermission::Write => write!(f, "PROJECT_WRITE"),
            ProjectPermission::Admin => write!(f, "PROJECT_ADMIN"),
        }
    }
}

impl Client {
    pub fn project_default_permission(&self, project_key: &str) -> Result<String, Error> {
        let permissions = vec!["PROJECT_READ", "PROJECT_WRITE", "PROJECT_ADMIN"];

        for permission in permissions {
            let url = format!(
                "rest/api/1.0/projects/{}/permissions/{}/all",
                project_key, permission
            );
            let status: PermissionStatus = self.get(&url)?;
            if status.permitted {
                return Ok(permission.to_owned());
            }
        }

        Ok("PROJECT_NONE".to_owned())
    }
}

impl<'de> Deserialize<'de> for ProjectPermission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

#[derive(Debug, Serialize, PartialEq, Eq, Hash)]
pub enum RepositoryPermission {
    Read,
    Write,
    Admin,
}

impl FromStr for RepositoryPermission {
    type Err = Error;

    fn from_str(s: &str) -> Result<RepositoryPermission, Error> {
        match s {
            "REPO_READ" => Ok(RepositoryPermission::Read),
            "REPO_WRITE" => Ok(RepositoryPermission::Write),
            "REPO_ADMIN" => Ok(RepositoryPermission::Admin),
            _ => bail!("unexpected permission {}", s),
        }
    }
}

impl fmt::Display for RepositoryPermission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RepositoryPermission::Read => write!(f, "REPO_READ"),
            RepositoryPermission::Write => write!(f, "REPO_WRITE"),
            RepositoryPermission::Admin => write!(f, "REPO_ADMIN"),
        }
    }
}

impl<'de> Deserialize<'de> for RepositoryPermission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

#[derive(Debug, Serialize, PartialEq, Eq, Hash)]
pub enum GlobalPermission {
    Read,
    Write,
    Admin,
}

impl FromStr for GlobalPermission {
    type Err = Error;

    fn from_str(s: &str) -> Result<GlobalPermission, Error> {
        match s {
            "READ" => Ok(GlobalPermission::Read),
            "WRITE" => Ok(GlobalPermission::Write),
            "ADMIN" => Ok(GlobalPermission::Admin),
            _ => bail!("unexpected permission {}", s),
        }
    }
}

impl fmt::Display for GlobalPermission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GlobalPermission::Read => write!(f, "READ"),
            GlobalPermission::Write => write!(f, "WRITE"),
            GlobalPermission::Admin => write!(f, "ADMIN"),
        }
    }
}

impl<'de> Deserialize<'de> for GlobalPermission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}
