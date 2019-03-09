#[allow(dead_code)]
#[allow(unused)]

extern crate futures;
extern crate tokio;
extern crate tokio_ping;
extern crate termion;

use std::io::{Write, stdout, stdin};

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{color, clear, cursor};
             
//use tui::backend::TermionBackend;
//use tui::layout::{Constraint, Layout};
//use tui::style::{Color, Modifier, Style};
//use tui::widgets::{Block, Borders, Row, Table, Widget};
//use tui::Terminal;

//use crate::util::event::{Event, Events};

use futures::{Future, Stream};

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    writeln!(stdout,"{}", clear::All);
    writeln!(stdout,"{}", cursor::Goto(4,4));
    writeln!(stdout,"○○○○○○○○○○");
    writeln!(stdout,"{}", cursor::Goto(4,5));
    writeln!(stdout,"○○○○○○○○○○");
    writeln!(stdout,"{}", cursor::Goto(4,6));
    writeln!(stdout,"○○○○○○○○○○");
    writeln!(stdout,"{}", cursor::Goto(4,6));
    writeln!(stdout,"○○○○○○○○○○");


}
