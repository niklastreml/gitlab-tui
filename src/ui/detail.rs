use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{ActiveBlock, App};
use crate::ui::types::*;

use super::main_ui::get_color;

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

fn draw_details<B>(
    f: &mut Frame<B>,
    id_prefix: &str,
    item: Details,
    scroll_offset: (u16, u16),
    highlight_state: (bool, bool),
    word_wrap: bool,
    layout_chunk: Rect,
) where
    B: Backend,
{
    let mut text = vec![
        Spans::from(vec![
            Span::raw(id_prefix),
            Span::raw(item.iid.to_string()),
            Span::raw(" "),
            Span::styled(item.title, Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Spans::from(vec![Span::raw(render_line(layout_chunk))]),
        Spans::from(vec![
            Span::raw("Status ".to_string()),
            Span::styled(
                item.state.to_string(),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(item.state.into()),
            ),
        ]),
        Spans::from(vec![
            Span::raw("Created at ".to_string()),
            Span::styled(
                item.created_at.to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
        Spans::from(vec![
            Span::raw("Updated at ".to_string()),
            Span::styled(
                item.updated_at.to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
        Spans::from(vec![
            Span::raw("Created by ".to_string()),
            Span::styled(
                item.author.name,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
        Spans::from(vec![Span::raw(render_line(layout_chunk))]),
    ];

    text.extend(
        item.description
            .lines()
            .map(|l| Spans::from(Span::styled(l.to_string(), Style::default()))),
    );

    let mut b = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(get_color(highlight_state)))
                .title("Details"),
        )
        .scroll(scroll_offset);

    if word_wrap {
        b = b.wrap(Wrap { trim: false });
    }

    f.render_widget(b, layout_chunk)
}

pub fn draw_issue_details<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let issue = app.issues[app.selected_issue.unwrap_or(0)].clone();

    draw_details(
        f,
        "#",
        issue.into(),
        app.scroll_offset,
        (
            app.active_block == Some(ActiveBlock::Details),
            app.highlighted_block == ActiveBlock::Details,
        ),
        app.word_wrap,
        layout_chunk,
    );
}

fn render_line(rect: Rect) -> String {
    let mut s = String::new();
    for _ in 0..rect.width - 2 {
        s.push('-');
    }
    s
}

pub fn draw_mr_details<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let mr = app.mrs[app.selected_mr.unwrap_or(0)].clone();

    draw_details(
        f,
        "!",
        mr.into(),
        app.scroll_offset,
        (
            app.active_block == Some(ActiveBlock::Details),
            app.highlighted_block == ActiveBlock::Details,
        ),
        app.word_wrap,
        layout_chunk,
    );
}
