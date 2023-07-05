use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{ActiveBlock, App, Route};

use super::detail;

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.size());

    draw_sidebar(f, &app, layout[0]);
    draw_details(f, &app, layout[1]);
}

pub fn draw_details<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    match app.route {
        Route::Root => detail::draw_root(f, app, layout_chunk),
        Route::IssuesDetail => detail::draw_issue_details(f, app, layout_chunk),
        Route::MergeRequestDetail => detail::draw_mr_details(f, app, layout_chunk),
    }
}
pub fn draw_sidebar<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layout_chunk);

    let issues: Vec<String> = app
        .issues
        .iter()
        .map(|issue| issue.title.to_owned())
        .collect();
    let mrs: Vec<String> = app.mrs.iter().map(|mr| mr.title.to_owned()).collect();

    draw_selectable_list(
        f,
        app,
        "Issues",
        &issues,
        layout[0],
        (
            app.active_block == Some(ActiveBlock::IssueList),
            app.highlighted_block == ActiveBlock::IssueList,
        ),
        app.selected_issue,
    );
    draw_selectable_list(
        f,
        app,
        "Merge Requests",
        &mrs,
        layout[1],
        (
            app.active_block == Some(ActiveBlock::MRList),
            app.highlighted_block == ActiveBlock::MRList,
        ),
        app.selected_mr,
    );
}

pub fn draw_selectable_list<B, S>(
    f: &mut Frame<B>,
    _app: &App,
    title: &str,
    items: &[S],
    layout_chunk: Rect,
    highlight_state: (bool, bool),
    selected_index: Option<usize>,
) where
    B: Backend,
    S: std::convert::AsRef<str>,
{
    let mut state = ListState::default();
    state.select(selected_index);

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|i| ListItem::new(Span::raw(i.as_ref())))
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(get_color(highlight_state))),
        )
        .highlight_style(
            Style::default()
                .fg(get_color(highlight_state))
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, layout_chunk, &mut state);
}

fn get_color(highlight_state: (bool, bool)) -> Color {
    let (active, highlight) = highlight_state;
    if active {
        return Color::Green;
    }
    if highlight {
        return Color::Magenta;
    }

    Color::White
}
