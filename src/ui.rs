use bdays::HolidayCalendar;
use chrono::Duration;
use ratatui::{
    layout::Alignment,
    prelude::*,
    style::{Color, Style, Stylize},
    widgets::{
        block::Title, Axis, BarChart, Block, BorderType, Borders, Chart, Dataset, GraphType,
        LegendPosition, Paragraph, block::Padding
    },
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let style: Style = Style::default().fg(Color::White).bg(Color::Black);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(main_layout[0]);

    let info = create_info(&app, style);
    let bar_chart = create_bar_chart(&app, style);
    let line_chart = create_line_chart(&app, style);

    frame.render_widget(info, top_layout[0]);
    frame.render_widget(bar_chart, top_layout[1]);
    frame.render_widget(line_chart, main_layout[1]);
}

fn create_info(app: &App, style: Style) -> Paragraph {
    // TODO:
    // move this to app.rs
    let hours_left = 1680 - app.total_hours.round() as i32;
    let days_left = hours_left / 8;

    let cal = bdays::calendars::WeekendsOnly;

    let today = chrono::offset::Local::now().date_naive();
    let completed = today + Duration::days(days_left.into());
    let completed_actual = cal.to_bday(completed, false);

    let text = format!(
        "Current hours: {}\n\n\
        Hours left until target is reached: {}\n\n\
        Based on 40-hour workweek and no vacations,\nyou will hit your target on: {}",
        app.total_hours.round(), hours_left, completed_actual);

    let paragraph = Paragraph::new(text)
        .block(
            Block::bordered()
                .title("Quick info")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(2, 0, 2, 0))
        )
        .style(style)
        .left_aligned();

    paragraph
}

fn create_bar_chart(app: &App, style: Style) -> BarChart {
    // TODO:
    // clean this shit up
    let data: Vec<(&str, u64)> = app.customer_hours_division.iter().map(|(s, n)| (&s[..], *n)).collect();

    // TODO:
    // fix sizes for smaller resolutions
    BarChart::default()
        .block(
            Block::default()
                .title("Hours per customer")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        )
        .style(style)
        .bar_width(12)
        .bar_gap(5)
        .bar_style(Style::new().yellow())
        .value_style(Style::new().yellow())
        .label_style(Style::new().white())
        .data(&data)
}

fn create_line_chart(app: &App, style: Style) -> Chart {
    let labels_x: Vec<Span> = (0..12).map(|x| Span::raw((x + 1).to_string())).collect();
    let bounds_y_max = if app.total_hours > 1680.0 {
        app.total_hours * 1.2
    } else {
        (1680.0 * 1.2) as f64
    };

    // TODO:
    // add seperate label for hour target...
    let labels_y = (0..(bounds_y_max) as i32)
        .step_by((bounds_y_max / 7.0) as usize)
        .map(|y| Span::raw(y.to_string()))
        .collect();

    let mut datasets = vec![Dataset::default()
        .name("Registered Hours".italic())
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(&app.cumulative_hours)];

    datasets.push(
        Dataset::default()
            .name("Yearly Hour Target".italic())
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Green))
            .graph_type(GraphType::Line)
            .data(&app.hour_target),
    );

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(
                    Title::default()
                        .content("Line chart total hours cumulative".white())
                        .alignment(Alignment::Center),
                )
                .borders(Borders::ALL),
        )
        .style(style)
        .x_axis(
            Axis::default()
                .title("Months")
                .style(Style::default().gray())
                .bounds([1.0, 366.0])
                .labels(labels_x),
        )
        .y_axis(
            Axis::default()
                .title("Hours")
                .style(Style::default().gray())
                .bounds([0.0, bounds_y_max])
                .labels(labels_y),
        )
        .legend_position(Some(LegendPosition::BottomRight))
        .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

    chart
}
