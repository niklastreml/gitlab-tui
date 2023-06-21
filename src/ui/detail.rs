use gitlab::{types::Issue, IssueState};
use ratatui::style::Color;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_root<B>(f: &mut Frame<B>, _app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let text = "Hello world".to_string();
    let b = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title("Gitlab"),
    );

    f.render_widget(b, layout_chunk)
}
pub fn draw_issue_details<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let issue = app.issues[app.selected_issue.unwrap_or(0)].clone();

    let mut text = vec![
        Spans::from(vec![
            Span::raw("#"),
            Span::raw(issue.iid.to_string()),
            Span::raw(" "),
            Span::styled(issue.title, Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Spans::from(vec![Span::raw(render_line(layout_chunk))]),
        Spans::from(vec![
            Span::raw("Status ".to_string()),
            Span::styled(
                render_issue_status(issue.state),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(get_issue_color(issue.state)),
            ),
        ]),
        Spans::from(vec![
            Span::raw("Created at ".to_string()),
            Span::styled(
                issue.created_at.to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
        Spans::from(vec![
            Span::raw("Updated at ".to_string()),
            Span::styled(
                issue.updated_at.to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
        Spans::from(vec![
            Span::raw("Created by ".to_string()),
            Span::styled(
                issue.author.name,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
        Spans::from(vec![Span::raw(render_line(layout_chunk))]),
    ];

    text.extend(
        issue
            .description
            .unwrap_or("".to_string())
            .lines()
            .map(|l| Spans::from(Span::styled(l.to_string(), Style::default()))),
    );

    let b = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title("Details"),
    );

    f.render_widget(b, layout_chunk)
}

fn render_issue_status(state: IssueState) -> String {
    match state {
        IssueState::Opened => "Opened".to_string(),
        IssueState::Closed => "Closed".to_string(),
        IssueState::Reopened => "Reopened".to_string(),
    }
}
fn get_issue_color(state: IssueState) -> Color {
    match state {
        IssueState::Opened => Color::Green,
        IssueState::Closed => Color::Gray,
        IssueState::Reopened => Color::LightGreen,
    }
}
fn render_line(rect: Rect) -> String {
    let mut s = String::new();
    for _ in 0..rect.width {
        s.push('-');
    }
    s
}

pub fn draw_mr_details<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let text = "This is the merge request details page".to_string();
    let b = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title("Details"),
    );

    f.render_widget(b, layout_chunk)
}
