use auth::permission::{GlobalPermission, ProjectPermission, RepositoryPermission};
use client;
use client::Client;
use failure::Error;
use group::Group;
use project::Project;
use repository::Repository;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::iter::FromIterator;
use user::User;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct GroupAccess<P> {
    group: Group,
    permission: P,
}

impl<P> fmt::Display for GroupAccess<P>
where
    P: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} has {}", self.group, self.permission)
    }
}

impl<P> GroupAccess<P> {
    pub fn group(&self) -> &Group {
        &self.group
    }

    pub fn permission(&self) -> &P {
        &self.permission
    }
}

impl Client {
    pub fn group_access(&self) -> Result<Vec<GroupAccess<GlobalPermission>>, Error> {
        let url = "rest/api/1.0/admin/permissions/groups";

        self.get_paged(&url)
    }

    pub fn project_group_access(
        &self,
        project_key: &str,
    ) -> Result<Vec<GroupAccess<ProjectPermission>>, Error> {
        let url = format!("rest/api/1.0/projects/{}/permissions/groups", project_key);

        self.get_paged(&url)
    }

    pub fn set_project_group_access(
        &self,
        project_key: &str,
        groups: Vec<GroupAccess<ProjectPermission>>,
    ) -> Result<(), Error> {
        let new_groups: HashSet<GroupAccess<ProjectPermission>> = HashSet::from_iter(groups);
        let current_groups = HashSet::from_iter(self.project_group_access(&project_key)?);

        for group in current_groups.difference(&new_groups) {
            println!("Deleting {}", group);
            let url = format!(
                "rest/api/1.0/projects/{}/permissions/groups?name={}",
                project_key,
                client::percent_encode(&group.group().name())
            );
            self.delete(&url)?;
        }

        for group in new_groups.difference(&current_groups) {
            println!("Adding {}", group);
            let url = format!(
                "rest/api/1.0/projects/{}/permissions/groups?permission={}&name={}",
                project_key,
                client::percent_encode(&group.permission().to_string()),
                client::percent_encode(group.group().name())
            );
            self.put::<GroupAccess<ProjectPermission>>(&url, None)?;
        }

        Ok(())
    }

    pub fn repository_group_access(
        &self,
        project_key: &str,
        repository_slug: &str,
    ) -> Result<Vec<GroupAccess<RepositoryPermission>>, Error> {
        let url = format!(
            "rest/api/1.0/projects/{}/repos/{}/permissions/groups",
            project_key, repository_slug
        );

        self.get_paged(&url)
    }

    pub fn set_repository_group_access(
        &self,
        project_key: &str,
        repository_slug: &str,
        groups: Vec<GroupAccess<RepositoryPermission>>,
    ) -> Result<(), Error> {
        let new_groups: HashSet<GroupAccess<RepositoryPermission>> = HashSet::from_iter(groups);
        let current_groups =
            HashSet::from_iter(self.repository_group_access(&project_key, &repository_slug)?);

        for group in current_groups.difference(&new_groups) {
            println!("Deleting {}", group);
            let url = format!(
                "rest/api/1.0/projects/{}/repos/{}/permissions/groups?name={}",
                project_key,
                repository_slug,
                client::percent_encode(&group.group().name())
            );
            self.delete(&url)?;
        }

        for group in new_groups.difference(&current_groups) {
            println!("Adding {}", group);
            let url = format!(
                "rest/api/1.0/projects/{}/repos/{}/permissions/groups?permission={}&name={}",
                project_key,
                repository_slug,
                client::percent_encode(&group.permission().to_string()),
                client::percent_encode(group.group().name())
            );
            self.put::<GroupAccess<RepositoryPermission>>(&url, None)?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct UserAccess<P> {
    user: User,
    permission: P,
}

impl<P> fmt::Display for UserAccess<P>
where
    P: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} has {}", self.user, self.permission)
    }
}

impl<P> UserAccess<P> {
    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn permission(&self) -> &P {
        &self.permission
    }
}

impl Client {
    pub fn project_user_access(
        &self,
        project_key: &str,
    ) -> Result<Vec<UserAccess<ProjectPermission>>, Error> {
        let url = format!("rest/api/1.0/projects/{}/permissions/users", project_key);

        self.get_paged(&url)
    }

    pub fn set_project_user_access(
        &self,
        project_key: &str,
        users: Vec<UserAccess<ProjectPermission>>,
    ) -> Result<(), Error> {
        let new_users: HashSet<UserAccess<ProjectPermission>> = HashSet::from_iter(users);
        let existing_users = HashSet::from_iter(self.project_user_access(&project_key)?);

        for user in existing_users.difference(&new_users) {
            println!("Deleting {}", user);
            let url = format!(
                "rest/api/1.0/projects/{}/permissions/users?name={}",
                project_key,
                client::percent_encode(&user.user().name())
            );
            self.delete(&url)?;
        }

        for user in new_users.difference(&existing_users) {
            println!("Adding {}", user);
            let url = format!(
                "rest/api/1.0/projects/{}/permissions/users?permission={}&name={}",
                project_key,
                client::percent_encode(&user.permission().to_string()),
                client::percent_encode(user.user().name())
            );
            self.put::<UserAccess<ProjectPermission>>(&url, None)?;
        }

        Ok(())
    }

    pub fn repository_user_access(
        &self,
        project_key: &str,
        repository_slug: &str,
    ) -> Result<Vec<UserAccess<RepositoryPermission>>, Error> {
        let url = format!(
            "rest/api/1.0/projects/{}/repos/{}/permissions/users",
            project_key, repository_slug
        );

        self.get_paged(&url)
    }

    pub fn set_repository_user_access(
        &self,
        project_key: &str,
        repository_slug: &str,
        users: Vec<UserAccess<RepositoryPermission>>,
    ) -> Result<(), Error> {
        let new_users: HashSet<UserAccess<RepositoryPermission>> = HashSet::from_iter(users);
        let existing_users =
            HashSet::from_iter(self.repository_user_access(&project_key, &repository_slug)?);

        for user in existing_users.difference(&new_users) {
            println!("Deleting {}", user);
            let url = format!(
                "rest/api/1.0/projects/{}/repos/{}/permissions/users?name={}",
                project_key,
                repository_slug,
                client::percent_encode(&user.user().name())
            );
            self.delete(&url)?;
        }

        for user in new_users.difference(&existing_users) {
            println!("Adding {}", user);
            let url = format!(
                "rest/api/1.0/projects/{}/repos/{}/permissions/users?permission={}&name={}",
                project_key,
                repository_slug,
                client::percent_encode(&user.permission().to_string()),
                client::percent_encode(user.user().name())
            );
            self.put::<UserAccess<RepositoryPermission>>(&url, None)?;
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryKeyAccess {
    key: SshKey,
    repository: Repository,
    permission: String,
}

impl fmt::Display for RepositoryKeyAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} has {}", self.key, self.permission)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectKeyAccess {
    key: SshKey,
    project: Project,
    permission: String,
}

impl fmt::Display for ProjectKeyAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} has {}", self.key, self.permission)
    }
}

impl Client {
    pub fn project_key_accesses(&self, project_key: &str) -> Result<Vec<ProjectKeyAccess>, Error> {
        let url = format!("rest/keys/1.0/projects/{}/ssh", project_key);

        self.get_paged(&url)
    }

    pub fn repository_key_accesses(
        &self,
        project_key: &str,
        repository_slug: &str,
    ) -> Result<Vec<RepositoryKeyAccess>, Error> {
        let url = format!(
            "rest/keys/1.0/projects/{}/repos/{}/ssh",
            project_key, repository_slug
        );

        self.get_paged(&url)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SshKey {
    id: u64,
    text: String,
    label: String,
}

impl fmt::Display for SshKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}
