// use gitlab;
use gitlab::{types::Issue, MergeRequest, ProjectId, UserBasic, UserId, UserState};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ActiveBlock {
    IssueList,
    MRList,
}
#[derive(Debug)]
pub struct App {
    pub issues: Vec<Issue>,
    pub mrs: Vec<MergeRequest>,
    pub highlighted_block: ActiveBlock,
    pub active_block: Option<ActiveBlock>,
    pub selected_issue: Option<usize>,
    pub selected_mr: Option<usize>,
    pub active_git_remote: Option<String>,
    pub route: Route,
    pub request_queue: Vec<Task>,
}

#[derive(Debug)]
pub enum Task {
    UpdateIssues,
    UpdateMRs,
    FetchIssue(i32),
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            issues: vec![],
            mrs: vec![],
            highlighted_block: ActiveBlock::IssueList,
            active_block: None,
            selected_issue: None,
            selected_mr: None,
            route: Route::Root,
            active_git_remote: None,
            request_queue: vec![],
        }
    }

    pub fn next_issue(&mut self) {
        if self.issues.len() == 0 {
            return;
        }
        match self.selected_issue {
            None => {
                if self.issues.len() > 0 {
                    self.selected_issue = Some(0);
                }
            }
            Some(i) => self.selected_issue = Some((i + 1) % self.issues.len()),
        }
    }
    pub fn prev_issue(&mut self) {
        if self.issues.len() == 0 {
            return;
        }
        match self.selected_issue {
            None => {
                if self.issues.len() > 0 {
                    self.selected_issue = Some(0);
                }
            }
            Some(0) => self.selected_issue = Some(self.issues.len() - 1),
            Some(i) => self.selected_issue = Some((i - 1) % self.issues.len()),
        }
    }

    pub fn next_mr(&mut self) {
        if self.mrs.len() == 0 {
            return;
        }
        match self.selected_mr {
            None => {
                if self.issues.len() > 0 {
                    self.selected_mr = Some(0);
                }
            }
            Some(i) => self.selected_mr = Some((i + 1) % self.mrs.len()),
        }
    }
    pub fn prev_mr(&mut self) {
        if self.mrs.len() == 0 {
            return;
        }
        match self.selected_mr {
            None => {
                if self.mrs.len() > 0 {
                    self.selected_mr = Some(0);
                }
            }
            Some(0) => self.selected_mr = Some(self.mrs.len() - 1),
            Some(i) => self.selected_mr = Some((i - 1) % self.mrs.len()),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Route {
    Root,
    // holds the id of the selected issue
    IssuesDetail,
    // holds the id of the selected mr
    MergeRequestDetail,
}
