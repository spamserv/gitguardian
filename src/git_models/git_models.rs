use serde::{Deserialize, Serialize};


#[derive(serde::Serialize)]
pub struct UpdateFileRequest<'a> {
    pub message: &'a str,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFileResponse {
    commit: CommitInfo,
    content: Option<FileContent>,
}

#[derive(Debug, Deserialize)]
pub struct CommitInfo {
    sha: String,
}

#[derive(Debug, Deserialize)]
pub struct FileContent {
    sha: String,
    // plus any other fields you might need
}

#[derive(serde::Serialize)]
pub struct CreateRef {
    // GitHub expects the full ref format, e.g. "refs/heads/my-new-branch"
    #[serde(rename = "ref")]
    pub ref_: String,
    pub sha: String,
}

#[derive(Serialize)]
pub struct CreateReviewRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<ReviewComment<'a>>>,
}

#[derive(Serialize)]
pub struct ReviewComment<'a> {
    pub path: &'a str, // e.g. "src/lib.rs"
    pub position: u32, // The line index in the diff to comment on
    pub body: &'a str, // The actual comment text
}

#[derive(Serialize)]
pub struct UpdateRepo {
    pub delete_branch_on_merge: bool,
}