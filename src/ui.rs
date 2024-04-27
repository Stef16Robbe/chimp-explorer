use ratatui::{
    layout::Alignment,
    prelude::*,
    style::{Color, Style, Stylize},
    widgets::{
        block::Title, Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, LegendPosition,
        Paragraph,
    },
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.size());

    let title = create_title(&app);
    let chart = create_line_chart(&app);

    frame.render_widget(title, layout[0]);
    frame.render_widget(chart, layout[1]);
}

fn create_title(app: &App) -> Paragraph {
    let paragraph = Paragraph::new(format!("Total hours: {}", app.total_hours.round()))
        .block(
            Block::bordered()
                .title("Template")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .centered();

    paragraph
}

fn create_line_chart(app: &App) -> Chart {
    let labels_x: Vec<Span> = (0..12).map(|x| Span::raw((x + 1).to_string())).collect();
    let labels_y = (0..(app.total_hours * 1.2) as i32)
        .step_by((app.total_hours / 10.0) as usize)
        .map(|y| Span::raw(y.to_string()))
        .collect();

    let datasets = vec![Dataset::default()
        .name("Registered Hours".italic())
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(&app.cumulative_hours)];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(
                    Title::default()
                        .content("Line chart total hours cumulative".cyan().bold())
                        .alignment(Alignment::Center),
                )
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("Months")
                .style(Style::default().gray())
                .bounds([1.0, 365.0])
                .labels(labels_x),
        )
        .y_axis(
            Axis::default()
                .title("Hours")
                .style(Style::default().gray())
                .bounds([0.0, app.total_hours * 1.2])
                .labels(labels_y),
        )
        .legend_position(Some(LegendPosition::TopLeft))
        .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

    chart
}
