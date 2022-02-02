use crate::parser;
use crate::util::StatefulList;

use std::io;
use termion::{event::Key, input::MouseTerminal, raw::RawTerminal, screen::AlternateScreen};

use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

// ----- [ End of use_module phase ] ----- //

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

/// helper function to create a centered rect using up
/// certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

type TerminalFrame<'a> =
    Frame<'a, TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<io::Stdout>>>>>;

pub struct App<'a> {
    items_left: StatefulList<String>,
    items_right: StatefulList<String>,
    input_mode: InputMode,
    parser: &'a parser::Parser,
    current_desc: String,
    show_help: bool,
    pub stop_render: bool,
}

impl<'a> App<'a> {
    pub fn new(parser: &'a parser::Parser) -> Self {
        let mut items_left = StatefulList::with_items(vec![]);
        let mut items_right = StatefulList::with_items(vec![]);

        for entry in &parser.entries {
            let i_name = String::from(&entry.name);
            let i_value = String::from(&entry.value);

            items_left.items.push(i_name);
            items_right.items.push(i_value);
        }

        App {
            items_left,
            items_right,
            input_mode: InputMode::Normal,
            parser,
            show_help: true,
            stop_render: false,
            current_desc: String::from(""),
        }
    }

    fn switch_input_mode(&mut self) {
        if self.input_mode == InputMode::Normal {
            return self.input_mode = InputMode::Editing;
        }

        if self.input_mode == InputMode::Editing {
            return self.input_mode = InputMode::Normal;
        }
    }

    fn run_cmake(&mut self) {
        let mut out_vec: Vec<String> = Vec::new();

        for i in &self.items_right.items {
            out_vec.push(i.clone());
        }

        let mut proc = self.parser.run_cmake(&out_vec);
        if let Err(_x) = proc.spawn() {
            // TODO : handle
        }

        self.stop_render = true;
    }

    pub fn render(&mut self, f: &mut TerminalFrame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
            .split(f.size());

        self.render_main(f, chunks[0]);
        self.render_description(f, chunks[1]);
        self.render_help(f);
    }

    fn render_main(&mut self, f: &mut TerminalFrame, area: Rect) {
        let gray_bg = Color::Rgb(193, 193, 193);
        let gray_fg = Color::Rgb(50, 50, 50);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(area);

        let items: Vec<ListItem> = self
            .items_left
            .items
            .iter()
            .map(|i| {
                let lines = vec![Spans::from(String::from(i))];
                ListItem::new(lines).style(Style::default())
            })
            .collect();

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                "Properties",
                Style::default().add_modifier(Modifier::BOLD),
            )))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(gray_bg)
                    .fg(gray_fg),
            );

        f.render_stateful_widget(items, chunks[0], &mut self.items_left.state);

        // items_right -> options
        let items_right: Vec<ListItem> = self
            .items_right
            .items
            .iter()
            .map(|i| {
                let lines = vec![Spans::from(String::from(i))];
                ListItem::new(lines).style(Style::default())
            })
            .collect();

        let items_right = List::new(items_right)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                "Values",
                Style::default().add_modifier(Modifier::BOLD),
            )))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(gray_bg)
                    .fg(gray_fg),
            );
        f.render_stateful_widget(items_right, chunks[1], &mut self.items_right.state);
    }

    fn render_description(&mut self, f: &mut TerminalFrame, area: Rect) {
        let text = vec![Spans::from(self.current_desc.clone())];

        let create_block = |title| {
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title(Span::styled(
                    title,
                    Style::default().add_modifier(Modifier::BOLD),
                ))
        };

        let paragraph = Paragraph::new(text)
            .style(Style::default())
            .block(create_block("Description"))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }

    fn render_help(&mut self, f: &mut TerminalFrame) {
        if self.show_help {
            let block = Block::default()
                .title(Span::styled(
                    "Help",
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL);

            let area = centered_rect(60, 20, f.size());

            let help_text = "<Arrows> - for navigating, <Enter> - to edit value, <Esc> - exit, <Space> - launch cmake, <h> - toggle help";

            let text = vec![Spans::from(help_text)];
            let text_wgt = Paragraph::new(text)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true })
                .block(block);

            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(text_wgt, area);
        }
    }

    fn select_next(&mut self) {
        self.items_left.next();
        self.items_right.next();
        self.update_desc();
    }

    fn select_previous(&mut self) {
        self.items_left.previous();
        self.items_right.previous();
        self.update_desc();
    }

    fn update_desc(&mut self) {
        if let Some(pos_sel) = self.items_right.state.selected() {
            self.current_desc = self.parser.entries[pos_sel].desc.clone();
        }
    }

    fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn handle_key(&mut self, key: &Key) {
        if self.input_mode == InputMode::Normal {
            match key {
                Key::Char('\n') => {
                    if let Some(pos_sel) = self.items_right.state.selected() {
                        if self.parser.entries[pos_sel].type_val == parser::CmakeTypeVal::Boolean {
                            if self.items_right.items[pos_sel] == "OFF" {
                                self.items_right.items[pos_sel] = String::from("ON");
                            } else {
                                self.items_right.items[pos_sel] = String::from("OFF");
                            }
                        } else {
                            self.switch_input_mode();
                        }
                    }
                }

                Key::Left => {}

                Key::Down => self.select_next(),

                Key::Up => self.select_previous(),

                Key::Char(' ') => {
                    self.run_cmake();
                }

                Key::Char('h') => self.toggle_help(),

                Key::Right => {}

                _ => (),
            }

            return;
        }

        if self.input_mode == InputMode::Editing {
            let sel = self.items_right.state.selected().unwrap();

            match key {
                Key::Char('\n') => {
                    //app.messages.push(app.input.drain(..).collect());
                    self.switch_input_mode();
                }
                Key::Char(c) => {
                    self.items_right.items[sel].push(*c);
                }
                Key::Backspace => {
                    self.items_right.items[sel].pop();
                }

                _ => (),
            }

            return;
        }
    }
}
