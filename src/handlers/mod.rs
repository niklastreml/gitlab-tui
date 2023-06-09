use crossterm::event::KeyEvent;

use crate::app::{ActiveBlock, App};

use self::{issuelist::handle_issuelist, mrlist::handle_mrlist, root::handle_root};

pub mod issuelist;
pub mod mrlist;
pub mod root;

pub fn handle_input(e: KeyEvent, app: &mut App) {
    match app.active_block {
        Some(ActiveBlock::IssueList) => handle_issuelist(e, app),
        Some(ActiveBlock::MRList) => handle_mrlist(e, app),
        None => handle_root(e, app),
    }
}
