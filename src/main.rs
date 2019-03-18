#[allow(dead_code)]
#[allow(unused)]

extern crate futures;
extern crate tokio;
extern crate tokio_ping;
extern crate termion;
extern crate rand;

use rand::Rng;

use std::collections::{VecDeque};
use std::io::{Write, stdout, stdin};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc,RwLock};
use std::thread;
use std::thread::{sleep};
use std::time;

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
pub struct PingData {
    addr: IpAddr,
    rtt: Arc<RwLock<i32>>
}

use futures::{Future, Stream};

//writeln!(stdout,"{}", cursor::Goto(4,8));
//writeln!(stdout,"○○○○○○○○○○");
fn draw(size: usize, ips: Vec<PingData>) {
    let symbols = " +o○";
    let mut stdout = stdout().into_raw_mode().unwrap();
    loop{
        writeln!(stdout,"{}", clear::All);
        for n in 0..(size * size) {
            //println!("{:#?}, {:#?}", (n % size) + 1, ( n as f32 / size as f32) as u16 + 1);
            writeln!(stdout,"{}", cursor::Goto((n % size) as u16 + 1, (n as f32 / size as f32) as u16 + 1) );
            let pd = *ips[n].clone().rtt.read().unwrap();
            //println!("loop-{:?}, {:?}, {:?}", n, ips[n].addr, pd);
            if      pd > 750 { writeln!(stdout,"{}",  symbols.chars().nth(3).unwrap() ); } 
            else if pd > 500 { writeln!(stdout,"{}",  symbols.chars().nth(2).unwrap() ); }
            else if pd > 250 { writeln!(stdout,"{}",  symbols.chars().nth(1).unwrap() ); }
            else             { writeln!(stdout,"{}",  symbols.chars().nth(0).unwrap() ); }
        }
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn pingLoop(pd: PingData) {
    loop {
        let duration = rand::thread_rng().gen_range(0,1000);
        //println!("{:?}, {:?}", pd.addr, duration);
        thread::sleep(time::Duration::from_millis(duration));
        let mut w = pd.rtt.write().unwrap();
        *w = duration as i32;
    }
}

fn main() {
    let size = 8;
    let mut ips: Vec<PingData> = Vec::with_capacity(size * size);
    for n in 0..(size * size) {
        ips.push(PingData{addr: IpAddr::V4(Ipv4Addr::new(0,0,0,n as u8)), rtt: Arc::new(RwLock::new(0))});
    }
    for ip in &ips {
        let myip = ip.clone();
        thread::spawn(move || {
            pingLoop(myip)
        });
    }
    draw(size, ips);
}
