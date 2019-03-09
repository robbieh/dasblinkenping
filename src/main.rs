#[allow(dead_code)]
#[allow(unused)]

extern crate futures;
extern crate tokio;

extern crate tokio_ping;

use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Row, Table, Widget};
use tui::Terminal;

use crate::util::event::{Event, Events};

use futures::{Future, Stream};

fn main() {


}
