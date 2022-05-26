use tui::layout::Rect;
use pnet::datalink::NetworkInterface;
use pnet::ipnetwork::IpNetwork;
use crate::windows::*;
use crate::State;
use crate::Window;
use crate::WindowCommand;
use crossterm::event::KeyCode;
use std::collections::HashMap;
use std::rc::Rc;
use tui::backend::Backend;
use tui::widgets::Paragraph;
use tui::Frame;

fn multiplayer_menu_window<B: 'static + Backend>(_: Rc<State>) -> Box<dyn Fn(&mut Frame<B>)> {
    Box::new(|f: &mut Frame<B>| {
        let game_title = "█▀▄▀█ █░█ █░░ ▀█▀ █ █▀█ █░░ ▄▀█ █▄█ █▀▀ █▀█\n█░▀░█ █▄█ █▄▄ ░█░ █ █▀▀ █▄▄ █▀█ ░█░ ██▄ █▀▄";

        let container = Layout::default()
            .horizontal_margin(f.size().width / 4)
            .vertical_margin(f.size().height / 4)
            .constraints([ Constraint::Percentage(1) ])
            .split(f.size());
        
        let buttons = vec![
            ("J", "oin Room"),
            ("N", "ew Room"),
        ];

        let padding = Padding { width: container[0].width / 5, height: container[0].height / 8 };

        create_menu_pad(f, container[0], game_title, buttons, padding);
        // let ip_address = local_ip_address::local_ip().unwrap();
        // let network_interfaces = local_ip_address::list_afinet_netifas().unwrap();

        // let local_netfias: Vec<Spans> = network_interfaces
        //     .iter()
        //     .map(|(name, ip)| Spans::from(vec![Span::from(format!("name: {}, ip: {}", name, ip))]))
        //     .collect();

        // let interfaces = pnet::datalink::interfaces();
        // let interfaces: Vec<&NetworkInterface> = interfaces
        //     .iter()
        //     .filter(|e| !e.ips.is_empty() && e.is_up())
        //     .collect();

        // let inter_spans: Vec<Spans> = interfaces
        //     .iter()
        //     .map(|&i|{
        //         Spans::from(vec![ Span::from(format!("name: {}, ip: {:?}", i.name, i.ips)) ])
        //     })
        //     .collect();
        // let text = vec![
        //     vec![Spans::from(vec![Span::from(format!(
        //         "local: {}",
        //         ip_address
        //     ))])],
        //     local_netfias,
        //     vec![
        //         Spans::from(vec![Span::from("-- INTERFACES --")])
        //     ],
        //     inter_spans
        // ];
        // let text = text.concat();
        // let par = Paragraph::new(text).alignment(Alignment::Center);
        // f.render_widget(par, f.size());

    })
}

pub fn create_multiplayer_menu_window<B: 'static + Backend>(_: &mut State) -> Option<Window<B>> {
    Some(Window {
        ui: multiplayer_menu_window,
        commands: HashMap::from([(
            KeyCode::Esc,
            WindowCommand {
                activator_key: KeyCode::Esc,
                action: Box::new(create_main_menu_window),
            },
        ),
        (
            KeyCode::Char('n'),
            WindowCommand::new_char_command('n', Box::new(create_mp_create_server_window))
        ),
        (
            KeyCode::Char('j'),
            WindowCommand::new_char_command('j', Box::new(create_mp_join_server_window))
        )]),
    })
}
