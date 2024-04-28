use ratatui::{
    layout::Alignment,
    prelude::*,
    style::{Color, Style, Stylize},
    widgets::{
        block::Title, Axis, BarChart, Block, BorderType, Borders, Chart, Dataset, GraphType,
        LegendPosition, Paragraph,
    },
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
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

    let title = create_title(&app);
    let line_chart = create_line_chart(&app);
    let bar_chart = create_bar_chart(&app);

    frame.render_widget(title, top_layout[0]);
    frame.render_widget(bar_chart, top_layout[1]);
    frame.render_widget(line_chart, main_layout[1]);
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

fn create_bar_chart(app: &App) -> BarChart {
    // TODO:
    // clean this shit up
    let data: Vec<(&str, u64)> = app.customer_hours_division.iter().map(|(s, n)| (&s[..], *n)).collect();

    BarChart::default()
        .block(
            Block::default()
                .title("Hours per customer")
                .borders(Borders::ALL),
        )
        .bar_width(15)
        .bar_gap(5)
        .bar_style(Style::new().yellow())
        .value_style(Style::new().yellow())
        .label_style(Style::new().white())
        .data(&data)
}

fn create_line_chart(app: &App) -> Chart {
    let labels_x: Vec<Span> = (0..12).map(|x| Span::raw((x + 1).to_string())).collect();
    let labels_y = (0..(app.total_hours * 1.2) as i32)
        .step_by((app.total_hours / 10.0) as usize)
        .map(|y| Span::raw(y.to_string()))
        .collect();
    let bounds_y_max = if app.total_hours > 1680.0 {
        app.total_hours * 1.2
    } else {
        (1680.0 * 1.2) as f64
    };

    let mut datasets = vec![Dataset::default()
        .name("Registered Hours".italic())
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(&app.cumulative_hours)];

    datasets.push(
        Dataset::default()
            .name("Yearly hour target".italic())
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
                .bounds([0.0, bounds_y_max])
                .labels(labels_y),
        )
        .legend_position(Some(LegendPosition::BottomRight))
        .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

    chart
}
