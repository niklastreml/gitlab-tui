use gitlab::Project;
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

pub fn draw_root<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let text = match &app.project {
        Some(project) => project_info(project, layout_chunk),
        None => vec![draw_metadata_entry(
            "Currently".to_string(),
            "Loading".to_string(),
        )],
    };
    let b = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title("Gitlab"),
    );

    f.render_widget(b, layout_chunk)
}

fn project_info<'a>(project: &Project, layout_chunk: Rect) -> Vec<Spans<'a>> {
    let mut spans = vec![
        Spans::from(vec![Span::raw(project.name_with_namespace.clone())]),
        Spans::from(draw_metadata_entry(
            "Link".to_string(),
            project.web_url.to_string(),
        )),
        Spans::from(draw_metadata_entry(
            "Stars".to_string(),
            project.star_count.to_string(),
        )),
        Spans::from(draw_metadata_entry(
            "Forks".to_string(),
            project.forks_count.to_string(),
        )),
    ];

    project
        .tag_list
        .iter()
        .map(|t| t.clone())
        .reduce(|prev, curr| format!("{}, {}", prev, curr))
        .map(|tl| spans.push(draw_metadata_entry("Tags".to_string(), tl)));

    project.description.clone().and_then(|desc| {
        spans.push(draw_metadata_entry("Description".to_string(), desc));
        None::<bool>
    });

    spans.push(Spans::from(vec![Span::raw(render_line(layout_chunk))]));

    spans
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
        draw_metadata_entry("Link".to_string(), item.url),
        draw_metadata_entry("Created at".to_string(), item.created_at.to_string()),
        draw_metadata_entry("Updated at".to_string(), item.updated_at.to_string()),
        draw_metadata_entry("Created by".to_string(), item.author.name),
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

fn draw_metadata_entry<'a>(key: String, value: String) -> Spans<'a> {
    Spans::from(vec![
        Span::raw(key + " "),
        Span::styled(value, Style::default().add_modifier(Modifier::BOLD)),
    ])
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
