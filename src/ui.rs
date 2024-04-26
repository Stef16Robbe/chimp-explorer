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
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    render_title(frame, app);
    render_line_chart(frame, app);
}

fn render_title(frame: &mut Frame, app: &mut App) {
    let data = Paragraph::new(format!("Total hours: {}", app.total_hours.round()))
        .block(
            Block::bordered()
                .title("Template")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .centered();

    frame.render_widget(data, frame.size())
}

fn render_line_chart(frame: &mut Frame, app: &mut App) {
    let labels_x: Vec<Span> = (0..12).map(|x| Span::raw((x + 1).to_string())).collect();
    let labels_y = (0..app.total_hours as i32)
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
                .bounds([1.0, 12.0])
                .labels(labels_x),
        )
        .y_axis(
            Axis::default()
                .title("Hours")
                .style(Style::default().gray())
                .bounds([0.0, app.total_hours])
                .labels(labels_y),
        )
        .legend_position(Some(LegendPosition::TopLeft))
        .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

    frame.render_widget(chart, frame.size());
}
