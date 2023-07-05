use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{ActiveBlock, App};

pub fn handle_details(e: KeyEvent, app: &mut App) {
    match e {
        KeyEvent {
            code: KeyCode::Char('j'),
            ..
        } => app.scroll_down(),
        KeyEvent {
            code: KeyCode::Char('k'),
            ..
        } => app.scroll_up(),
        KeyEvent {
            code: KeyCode::Char('h'),
            ..
        } => app.scroll_left(),
        KeyEvent {
            code: KeyCode::Char('l'),
            ..
        } => app.scroll_right(),
        KeyEvent {
            code: KeyCode::Esc, ..
        } => {
            app.active_block = app.previous_active_block;
            app.highlighted_block = app.previous_active_block.unwrap_or(ActiveBlock::IssueList);
            app.previous_active_block = None;
            app.reset_scroll();
        }
        _ => {}
    }
}
