use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{ActiveBlock, App, Route};

pub fn handle_issuelist(e: KeyEvent, app: &mut App) {
    match e {
        KeyEvent {
            code: KeyCode::Char('j'),
            ..
        } => app.next_issue(),
        KeyEvent {
            code: KeyCode::Char('k'),
            ..
        } => app.prev_issue(),
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => {
            app.active_block = Some(ActiveBlock::Details);
            app.route = Route::IssuesDetail;
            app.highlighted_block = ActiveBlock::Details;
        }
        KeyEvent {
            code: KeyCode::Esc, ..
        } => app.active_block = None,
        _ => {}
    }
}
