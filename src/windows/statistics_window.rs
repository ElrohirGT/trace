use tui::widgets::BarChart;
use tui::layout::Constraint;
use tui::layout::Layout;
use tui::widgets::Axis;
use tui::widgets::Chart;
use crate::get_track_record;
use tui::widgets::Dataset;
use tui::Frame;
use std::rc::Rc;
use std::collections::HashMap;
use crate::WindowCommand;
use crate::Window;
use crate::State;
use tui::backend::Backend;
use crate::windows::*;
use tui::symbols;
use tui::widgets::GraphType;
use tui::widgets::Block;
use tui::layout::Alignment;
use tui::layout::Direction;
use crossterm::event::KeyCode;

fn construct_line_chart<B: Backend>(f: &mut Frame<B>, point_series: &[(f64, f64)], wpm_series: &[(f64, f64)], accuracy_series: &[(f64, f64)], raw_data_length: usize) {
    let datasets = vec![
                Dataset::default()
                    .name("Points")
                    .marker(symbols::Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(Color::LightCyan))
                    .data(point_series),
                Dataset::default()
                    .name("WPM")
                    .marker(symbols::Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(Color::LightYellow))
                    .data(wpm_series),
                Dataset::default()
                    .name("Accuracy")
                    .marker(symbols::Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(Color::LightGreen))
                    .data(&accuracy_series),
            ];

            let filter = |n: usize| {
                let potential_step = (0.1*raw_data_length as f64) as usize;
                let step = if potential_step == 0 {1} else {potential_step};
                if n%step == 0 {Some(Span::from(n.to_string()))} else {None}
            };

            let chart = Chart::new(datasets)
                .block(Block::default().title("Statistics").title_alignment(Alignment::Center))
                .x_axis(
                    Axis::default()
                        .title(Span::styled("# Run", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)))
                        .bounds([0.0, raw_data_length as f64])
                        .labels((0..raw_data_length).filter_map(filter).collect())
                )
                .y_axis(
                    Axis::default()
                        .title(Span::styled("Press [TAB] to change", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)))
                        .bounds([0.0, 150.0])
                        .labels((0..151).filter_map(|n: u8| if n%10 == 0 {Some(Span::from(n.to_string()))} else {None}).collect())
                );
            f.render_widget(chart, f.size());
}

fn construct_bar_charts<B: Backend>(f: &mut Frame<B>, point_series: &[(f64, f64)], wpm_series: &[(f64, f64)], accuracy_series: &[(f64, f64)]) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30)
        ].as_ref())
        .split(f.size());

    let series = [point_series, wpm_series, accuracy_series];
    let titles = ["Points", "Series", "Accuracy"];
    let bar_styles = [
        Style::default().fg(Color::LightCyan),
        Style::default().fg(Color::LightYellow),
        Style::default().fg(Color::LightGreen),
    ];
    let value_styles = [
        Style::default().add_modifier(Modifier::BOLD),
        Style::default().add_modifier(Modifier::BOLD),
        Style::default().add_modifier(Modifier::BOLD),
    ];
    for i in 0..3 {
        let transformed_series: Vec<(String, u64)> = series[i]
            .into_iter()
            .map(|v| (v.0.to_string(), v.1 as u64))
            .collect();
        let result: Vec<(&str, u64)> = transformed_series
            .iter()
            .map(|v| (v.0.as_str(), v.1))
            .collect();
        let chart = BarChart::default()
            .block(Block::default().title(titles[i]).borders(Borders::BOTTOM))
            .bar_width(3)
            .bar_gap(1)
            .bar_style(bar_styles[i])
            .value_style(value_styles[i])
            .label_style(Style::default().fg(Color::White))
            .data(&result);
        f.render_widget(chart, layout[i]);
    }
}

fn statistics_window<B: 'static + Backend>(state: Rc<State>) -> Box<dyn Fn(&mut Frame<B>)> {
    Box::new(
        move |f: &mut Frame<B>| {
            let raw_data = get_track_record();
            
            let point_series: Vec<(f64, f64)> = (0..raw_data.len()).map(|i| (i as f64, raw_data[i].total_points)).collect();
            let wpm_series: Vec<(f64, f64)> = (0..raw_data.len()).map(|i| (i as f64, raw_data[i].wpm)).collect();
            let accuracy_series: Vec<(f64, f64)> = (0..raw_data.len()).map(|i| (i as f64, raw_data[i].accuracy * 100.0)).collect();

            if state.show_bar_charts{
                construct_bar_charts(f, &point_series, &wpm_series, &accuracy_series);
            } else {
                construct_line_chart(f, &point_series, &wpm_series, &accuracy_series, raw_data.len());
            }
        }
    )
}

pub fn create_statistics_window<B: 'static + Backend>(_: &mut State) -> Option<Window<B>> {
    Some(
        Window{
            ui: statistics_window,
            commands: HashMap::from([
                (
                    KeyCode::Esc,
                    WindowCommand{activator_key: KeyCode::Esc, action: Box::new(create_main_menu_window)}
                ),
                (
                    KeyCode::Tab,
                    WindowCommand { activator_key: KeyCode::Tab, action: Box::new(|s: &mut State| {
                        s.show_bar_charts = !s.show_bar_charts;
                        create_statistics_window(s)
                    })}
                )
            ])
        }
    )
}