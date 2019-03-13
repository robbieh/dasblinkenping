#[allow(dead_code)]
#[allow(unused)]

extern crate futures;
extern crate tokio;
extern crate tokio_ping;
extern crate termion;

use std::io::{Write, stdout, stdin};
use std::net::{IpAddr};
use std::sync::{Arc,RwLock};

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

#[derive(Debug,Clone)]
pub enum PingData {
    Addr(IpAddr),
    RTT(Arc<RwLock<i32>>)
}

use futures::{Future, Stream};

fn main() {
    let ips: Vec<IpAddr> = Vec::new();
    let mut stdout = stdout().into_raw_mode().unwrap();
    writeln!(stdout,"{}", clear::All);
    writeln!(stdout,"{}", cursor::Goto(4,8));
    writeln!(stdout,"○○○○○○○○○○");


}
