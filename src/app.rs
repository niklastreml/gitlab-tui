// use gitlab;
use gitlab::{types::Issue, MergeRequest, Project};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ActiveBlock {
    IssueList,
    MRList,
    Details,
}
#[derive(Debug)]
pub struct App {
    pub issues: Vec<Issue>,
    pub mrs: Vec<MergeRequest>,
    pub project: Option<Project>,
    pub highlighted_block: ActiveBlock,
    pub active_block: Option<ActiveBlock>,
    pub previous_active_block: Option<ActiveBlock>, // this should be done with a routing stack
    // probably
    pub selected_issue: Option<usize>,
    pub word_wrap: bool,
    pub selected_mr: Option<usize>,
    pub active_git_remote: String,
    pub route: Route,
    pub scroll_offset: (u16, u16),
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
            project: None,
            highlighted_block: ActiveBlock::IssueList,
            previous_active_block: None,
            active_block: None,
            word_wrap: false,
            selected_issue: None,
            selected_mr: None,
            route: Route::Root,
            active_git_remote: "origin".to_string(),
            scroll_offset: (0, 0),
        }
    }
    pub fn reset_scroll(&mut self) {
        self.scroll_offset = (0, 0);
    }
    pub fn scroll_up(&mut self) {
        if self.scroll_offset.0 == 0 {
            return;
        }
        self.scroll_offset.0 -= 1;
    }
    pub fn scroll_down(&mut self) {
        self.scroll_offset.0 += 1;
    }
    pub fn scroll_left(&mut self) {
        if self.scroll_offset.1 == 0 {
            return;
        }
        self.scroll_offset.1 -= 1;
    }
    pub fn scroll_right(&mut self) {
        self.scroll_offset.1 += 1;
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
