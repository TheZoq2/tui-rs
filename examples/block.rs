extern crate tui;
extern crate termion;

use std::io;
use termion::event;
use termion::input::TermRead;

use tui::{Terminal, TermionBackend};
use tui::widgets::{Widget, Block, border};
use tui::layout::{Group, Direction, Size};
use tui::style::Color;

fn main() {
    let mut terminal = Terminal::new(TermionBackend::new().unwrap()).unwrap();
    let stdin = io::stdin();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    draw(&mut terminal);
    for c in stdin.keys() {
        draw(&mut terminal);
        let evt = c.unwrap();
        if evt == event::Key::Char('q') {
            break;
        }
    }
    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<TermionBackend>) {

    let size = t.size().unwrap();

    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(7), Size::Min(0), Size::Fixed(7)])
        .render(t, &size, |t, chunks| {
            Block::default()
                .title("Top")
                .title_color(Color::Magenta)
                .background_color(Color::White)
                .border_color(Color::Magenta)
                .borders(border::BOTTOM)
                .render(t, &chunks[0]);
            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Fixed(7), Size::Min(0), Size::Fixed(7)])
                .render(t, &chunks[1], |t, chunks| {
                    Block::default().title("Left").title_color(Color::Yellow).render(t, &chunks[0]);
                    Block::default()
                        .title("Middle")
                        .title_color(Color::Cyan)
                        .border_color(Color::Cyan)
                        .borders(border::LEFT | border::RIGHT)
                        .render(t, &chunks[1]);
                    Block::default()
                        .title("Right")
                        .title_color(Color::Green)
                        .render(t, &chunks[2]);
                });
            Block::default()
                .title("Bottom")
                .title_color(Color::Red)
                .border_color(Color::Red)
                .borders(border::TOP)
                .render(t, &chunks[2]);
        });

    t.draw().unwrap();
}
