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
    pub route: Route,
}

impl Default for App {
    fn default() -> Self {
        App::new(
            vec![
                Issue::new(
                    ProjectId::new(12345),
                    "Weird bug in production".to_string(),
                    UserBasic {
                        username: "NiklasTreml".to_string(),
                        name: "Niklas Treml".to_string(),
                        avatar_url: None,
                        web_url: "".to_string(),
                        state: UserState::Active,
                        id: UserId::new(1),
                    },
                )
                .with_description("So production kind of died".to_string()),
                Issue::new(
                    ProjectId::new(12345),
                    "Fix everything".to_string(),
                    UserBasic {
                        username: "NiklasTreml".to_string(),
                        name: "Niklas Treml".to_string(),
                        avatar_url: None,
                        web_url: "".to_string(),
                        state: UserState::Active,
                        id: UserId::new(2),
                    },
                )
                .with_description("Everything is broken".to_string()),
            ],
            vec![],
        )
    }
}

impl App {
    pub fn new(issues: Vec<Issue>, mrs: Vec<MergeRequest>) -> Self {
        Self {
            issues,
            mrs,
            highlighted_block: ActiveBlock::IssueList,
            active_block: None,
            selected_issue: None,
            selected_mr: None,
            route: Route::Root,
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
