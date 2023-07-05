use std::fmt;

use chrono::Utc;
use gitlab::{Issue, IssueState, MergeRequest, MergeRequestState, UserBasic};
use ratatui::style::Color;

pub struct Details {
    pub iid: String,
    pub title: String,
    pub description: String,
    pub state: Status,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub author: UserBasic,
}

impl From<Issue> for Details {
    fn from(value: Issue) -> Self {
        Details {
            iid: value.iid.to_string(),
            title: value.title,
            description: value.description.unwrap_or("".to_string()),
            state: value.state.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            author: value.author,
        }
    }
}

impl From<MergeRequest> for Details {
    fn from(value: MergeRequest) -> Self {
        Details {
            iid: value.iid.to_string(),
            title: value.title,
            description: value.description.unwrap_or("".to_string()),
            state: value.state.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            author: value.author,
        }
    }
}

pub enum Status {
    Open,
    Opened,
    Closed,
    Reopened,
    Merged,
    Locked,
}
impl From<Status> for Color {
    fn from(value: Status) -> Self {
        match value {
            Status::Open => Color::Green,
            Status::Opened => Color::Green,
            Status::Closed => Color::Red,
            Status::Reopened => Color::Blue,
            Status::Merged => Color::Green,
            Status::Locked => Color::LightCyan,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Open => write!(f, "Open"),
            Status::Opened => write!(f, "Opened"),
            Status::Closed => write!(f, "Closed"),
            Status::Reopened => write!(f, "Reopened"),
            Status::Merged => write!(f, "Merged"),
            Status::Locked => write!(f, "Locked"),
        }
    }
}

impl From<MergeRequestState> for Status {
    fn from(value: MergeRequestState) -> Self {
        match value {
            MergeRequestState::Opened => Status::Opened,
            MergeRequestState::Closed => Status::Closed,
            MergeRequestState::Reopened => Status::Reopened,
            MergeRequestState::Merged => Status::Merged,
            MergeRequestState::Locked => Status::Locked,
        }
    }
}
impl From<IssueState> for Status {
    fn from(value: IssueState) -> Self {
        match value {
            IssueState::Opened => Status::Opened,
            IssueState::Closed => Status::Closed,
            IssueState::Reopened => Status::Reopened,
        }
    }
}
