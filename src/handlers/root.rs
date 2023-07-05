use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{ActiveBlock, App};

pub fn handle_root(e: KeyEvent, app: &mut App) {
    // use hjkl to change app.highlighted_block
    // use enter to set app.active_block to app.highlighted_block

    match e {
        KeyEvent {
            code: KeyCode::Char('j'),
            ..
        }
        | KeyEvent {
            code: KeyCode::Char('k'),
            ..
        } => {
            if app.highlighted_block == ActiveBlock::MRList {
                app.highlighted_block = ActiveBlock::IssueList
            } else {
                app.highlighted_block = ActiveBlock::MRList
            }
        }
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => app.active_block = Some(app.highlighted_block.clone()),
        _ => {}
    }
}
