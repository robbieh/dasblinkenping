#[allow(dead_code)]
#[allow(unused)]

extern crate futures;
extern crate tokio;
extern crate tokio_ping;
extern crate termion;
extern crate rand;

//use crate::util::event::{Event, Events};

use fastping_rs::Pinger;
use fastping_rs::PingResult::{Idle, Receive};

use futures::{Future, Stream};

use rand::Rng;

//use std::collections::{VecDeque};
//use std::io::{Write, stdout, stdin};
use std::io::{Write, stdout };
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc,RwLock};
use std::thread;
use std::time;

//use termion::event::Key;
//use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
//use termion::screen::AlternateScreen;
//use termion::{color, clear, cursor};
use termion::{cursor};
             
//use tui::backend::TermionBackend;
//use tui::layout::{Constraint, Layout};
//use tui::style::{Color, Modifier, Style};
//use tui::widgets::{Block, Borders, Row, Table, Widget};
//use tui::Terminal;


#[derive(Debug,Clone)]
pub struct PingData {
    addr: IpAddr,
    rtt: Arc<RwLock<i32>>
}


//writeln!(stdout,"{}", cursor::Goto(4,8));
//writeln!(stdout,"○○○○○○○○○○");
fn draw(size: usize, ips: Vec<PingData>) {
    let symbols = "⋅∘○◎● ";
    let s5 = symbols.chars().nth(5).unwrap();
    let s4 = symbols.chars().nth(4).unwrap();
    let s3 = symbols.chars().nth(3).unwrap();
    let s2 = symbols.chars().nth(2).unwrap();
    let s1 = symbols.chars().nth(1).unwrap();
    let s0 = symbols.chars().nth(0).unwrap();
    let mut stdout = stdout().into_raw_mode().unwrap();
    //writeln!(stdout,"{}", clear::All);
    loop{
        for n in 0..(size * size) {
            //println!("{:#?}, {:#?}", (n % size) + 1, ( n as f32 / size as f32) as u16 + 1);
            writeln!(stdout,"{}", cursor::Goto((n % size) as u16 + 1, (n as f32 / size as f32) as u16 + 1) ).expect("X");
            let pd = *ips[n].clone().rtt.read().unwrap();
            //println!("loop-{:?}, {:?}, {:?}", n, ips[n].addr, pd);
            if      pd > 100 { writeln!(stdout,"{}", s5).expect("X"); } 
            else if pd > 50  { writeln!(stdout,"{}", s4).expect("X"); } 
            else if pd > 25  { writeln!(stdout,"{}", s3).expect("X"); } 
            else if pd > 10  { writeln!(stdout,"{}", s2).expect("X"); }
            else if pd > 2   { writeln!(stdout,"{}", s1).expect("X"); }
            else             { writeln!(stdout,"{}", s0).expect("X"); }
        }
        thread::sleep(time::Duration::from_millis(10));
    }
}

fn do_ping(addr: IpAddr) -> i32 {
    let (mut pinger, results) = match Pinger::new(None,None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}",e)
    };
    pinger.add_ipaddr(&addr.to_string());
    pinger.ping_once();
    pinger.stop_pinger();
    match results.recv() {
        Ok(result) => {
            match result { 
                Idle{addr} => { 
                    //println!("Idle address: {}", addr); 
                    5000},
                Receive{addr, rtt} => { 
                    //println!("{} {}", addr, rtt.as_millis() as i32);
                    (rtt.as_millis()) as i32  }
            }
        },
        Err(_) => panic!("Could not run pinger"),
    }
}

fn ping_loop(pd: PingData) {
    loop {
        let duration = rand::thread_rng().gen_range(0,1000);
        //println!("{:?}, {:?}", pd.addr, duration);
        thread::sleep(time::Duration::from_millis(duration));
        let rtt = do_ping(pd.addr);
        let mut w = pd.rtt.write().unwrap();
        //*w = duration as i32;
        *w = rtt;
    }
}

fn main() {
    let size = 15;
    let mut ips: Vec<PingData> = Vec::with_capacity(size * size);
    for n in 0..(size * size) {
        ips.push(PingData{addr: IpAddr::V4(Ipv4Addr::new(192,168,2,n as u8)), rtt: Arc::new(RwLock::new(0))});
    }
    for ip in &ips {
        let myip = ip.clone();
        thread::spawn(move || {
            ping_loop(myip)
        });
    }
    draw(size, ips);
}
