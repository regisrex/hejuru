use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
 pub struct PullRequestEvent {
    action: String,
    number: u64,
    pull_request: PullRequest,
    repository: Repository,
    sender: User,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PullRequest {
    id: u64,
    url: String,
    title: String,
    user: User,
    body: Option<String>,
    created_at: String,
    updated_at: String,
    closed_at: Option<String>,
    merged_at: Option<String>,
    merge_commit_sha: Option<String>,
    head: Commit,
    base: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Repository {
    id: u64,
    name: String,
    full_name: String,
    owner: User,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct User {
    login: String,
    id: u64,
    email: String,
    avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Commit {
    sha: String,
    node_id: String,
    commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitDetails {
    author: User,
    committer: User,
    message: String,
}
