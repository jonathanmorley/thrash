/*- available merge strategies
 - merge commit
 - ff only
 - squash, ff-only
 - squash
 - ff

 - default reviewers
  - array of:
   - source
   - target
   - userlist
   - requires n approvals

   - PR process
    - require n approvers
    - require all approve
    - require all tasks done
    - require n successful builds
*/

use client::Client;
use failure::Error;
use itertools::Itertools;
use repository::Repository;
use std::fmt;
use user::User;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefaultReviewerGroup {
    id: u64,
    repository: Repository,
    source_ref_matcher: RefMatcher,
    target_ref_matcher: RefMatcher,
    reviewers: Vec<User>,
    required_approvals: u64,
}

impl fmt::Display for DefaultReviewerGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.reviewers.iter().join(", "))
    }
}

impl Client {
    pub fn default_reviewers(
        &self,
        project_key: &str,
        repository_slug: &str,
    ) -> Result<Vec<DefaultReviewerGroup>, Error> {
        let url = format!(
            "rest/default-reviewers/1.0/projects/{}/repos/{}/conditions",
            project_key, repository_slug
        );

        self.get(&url)
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RefMatcher {
    active: bool,
    id: String,
    display_id: String,
    #[serde(rename = "type")]
    _type: MatcherType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatcherType {
    id: String,
    name: String,
}
