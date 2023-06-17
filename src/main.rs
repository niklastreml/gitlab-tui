mod api;
mod app;
mod handlers;
mod ui;
use gitlab::Gitlab;
use std::{io, process::exit, sync::Arc};
use tokio::sync::Mutex;

use app::App;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (domain, namespace) = api::get_gitlab_remote("glab")?;
    let token = api::get_token(domain.clone())?;

    let api = {
        let this = gitlab::Gitlab::new(domain, token);
        match this {
            Ok(t) => t,
            Err(e) => panic!("Failed to connect to gitlab: {}", e),
        }
    };

    let app = Arc::new(Mutex::new(App::default()));
    // create app and run it
    match run_ui(&app.clone()) {
        Ok(_) => {}
        Err(e) => panic!("Failed to render UI: {}", e),
    }

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

#[tokio::main]
async fn run_ui(app: &Arc<Mutex<App>>) -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        let mut app = app.lock().await;
        terminal.draw(|f| ui::main_ui::draw_main_layout(f, &app))?;
        match event::read()? {
            Event::Key(key) => match key {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => break,
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => break,
                e => handlers::handle_input(e, &mut app),
            },
            _ => {}
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.clear()?;
    terminal.show_cursor()?;
    Ok(())
}
