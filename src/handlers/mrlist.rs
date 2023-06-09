use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Route};

pub fn handle_mrlist(e: KeyEvent, app: &mut App) {
    match e {
        KeyEvent {
            code: KeyCode::Char('j'),
            ..
        } => app.next_mr(),
        KeyEvent {
            code: KeyCode::Char('k'),
            ..
        } => app.prev_mr(),
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => app.route = Route::MergeRequestDetail,
        KeyEvent {
            code: KeyCode::Esc, ..
        } => app.active_block = None,
        _ => {}
    }
}
