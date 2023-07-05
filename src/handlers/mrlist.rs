use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{ActiveBlock, App, Route};

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
        } => {
            app.route = Route::MergeRequestDetail;
            app.active_block = Some(ActiveBlock::Details);
            app.highlighted_block = ActiveBlock::Details;
        }
        KeyEvent {
            code: KeyCode::Esc, ..
        } => app.active_block = None,
        _ => {}
    }
}
