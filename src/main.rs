#[allow(dead_code)]
#[allow(unused)]

#[macro_use] extern crate lazy_static;

extern crate cidr;
extern crate ctrlc;
extern crate lsystem;
extern crate rand;
extern crate regex;
extern crate termion;
extern crate fastping_rs;

mod hilbert;

use cidr::Cidr;
use cidr::IpCidr;
use fastping_rs::{Pinger,PingResult};
use fastping_rs::PingResult::{Idle, Receive};

use std::collections::{HashMap};
use std::env;
use std::io::{Write, stdout};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc};
use std::sync::mpsc::{channel,Sender,Receiver};
use std::sync::atomic::{AtomicBool,Ordering};
use std::time::Duration;
use std::thread;

use termion::raw::IntoRawMode;
use termion::{clear, cursor, style};
use termion::event::Key;
use termion::input::TermRead;

#[derive(Debug, Clone,Copy)]
pub enum MyPingResult {
    Idle{addr: IpAddr},
    Receive{addr: IpAddr, rtt: Duration},
}

#[derive(Debug, Clone)]
pub struct Params {
    ip_strings: Vec<String>,
}

#[derive(Clone,Copy,Debug)]
enum PingResultOrKey {
    Ping(MyPingResult),
    Key(Key)
}



#[derive(Debug, Clone)]
pub enum GuiState { Clear, Stats, Auto }
#[derive(Debug, Clone)]
pub enum CliState { Show, Hide }


fn keyboard_thread(tx: Sender<PingResultOrKey>){
    let stdin = std::io::stdin();
    for k in stdin.keys() {
        match k {
            Ok(k) => {tx.send(PingResultOrKey::Key(k)).unwrap();},
            Err(_) => {}
        };
    };
}

fn pinger_thread(tx: Sender<PingResultOrKey>,results: Receiver<fastping_rs::PingResult>) {
    loop {
        match results.recv() {
            Ok(result) => {
                match result {
                    PingResult::Receive{addr, rtt} => {
                        let mpr = MyPingResult::Receive{addr: addr, rtt: rtt};
                        tx.send(PingResultOrKey::Ping(mpr)).unwrap();
                    }
                    PingResult::Idle{addr} => {
                        let mpr = MyPingResult::Idle{addr: addr};
                        tx.send(PingResultOrKey::Ping(mpr)).unwrap();
                    }
                }
            },
            Err(_) => {}
        }
    }
}

fn expand_ip_cidr(ipcidr: &str, p: &mut Params){
    match cidr::IpCidr::from_str(ipcidr) {
        Ok(cidr) => {
            //println!("cidr: {:#?}", cidr);
            if cidr.is_ipv4() {
                let cidr4 = <IpCidr>::from(cidr);
                for ip in cidr4.iter() {
                    //println!("cidr ip: {:#?}", ip);
                    p.ip_strings.push(ip.to_string());
                }
            }
        },
        _ => { println!("not a cidr: {:#?}", ipcidr);}
    };
}

fn parse_args() -> Params {
    //let mut p = Params { ip_strings: Vec::<String>::new() };
    let mut p = Params { ip_strings: Vec::with_capacity(256), };
    for arg in env::args() {
        expand_ip_cidr(&arg, &mut p);
        //if arg.contains("*") { expand_ip_star(&arg, &mut p); }
    };
    p
}


fn main() {
    let p = parse_args();
    let mut cursor = 0; //cursor position on ip line

    //ctrlc
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || { r.store(false, Ordering::SeqCst); })
        .expect("Error setting Ctrl-C handler");

    let (pinger, results) = match Pinger::new(Some(10000),None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}",e)
    };

    let count = p.ip_strings.len();
    let sqrt = (count as f64).sqrt();
    let sqrt_again = (sqrt as f64).sqrt();
    let mut size = sqrt_again.ceil() as u16;
    size = (2.0_f64).powi(size as i32 ) as u16;
    let hilbert_points = hilbert::hilbert(sqrt_again.ceil() as isize);
    let mut ips_hash = HashMap::new();
    for n in 0..count {
        pinger.add_ipaddr(&p.ip_strings[n]);
        ips_hash.insert(&p.ip_strings[n], hilbert_points[n].clone());
    }

    pinger.run_pinger();
    let (proktx, prokrx) = channel();
    let p2= proktx.clone();
    thread::spawn(move || {pinger_thread(p2,results);});

    //set up keyboard input thread
    //let (kbdtx, kbdrx) = channel();
    thread::spawn(move || {keyboard_thread(proktx);});

    //let symbols = "∙∘⊙⊚● ";
    let symbols = "∙∘⊙⊚●-";
    //let symbols = "●⊚⊙∘∙ ";
    //let symbols = "●●∘∘∙ ";
    let s5 = symbols.chars().nth(5).unwrap();
    let s4 = symbols.chars().nth(4).unwrap();
    let s3 = symbols.chars().nth(3).unwrap();
    let s2 = symbols.chars().nth(2).unwrap();
    let s1 = symbols.chars().nth(1).unwrap();
    let s0 = symbols.chars().nth(0).unwrap();

    let mut stdout = stdout().into_raw_mode().unwrap();
    writeln!(stdout,"{}", clear::All).expect("Could not clear screen");
    writeln!(stdout,"{}", cursor::Hide).expect("Could not hide cursor");

    'mainloop: loop{
        if ! running.load(Ordering::SeqCst) { 
            write!(stdout,"{}", cursor::Goto(1, size + 1)).expect("X");
            pinger.stop_pinger();
            break; 
        };
        write!(stdout,"{}{}", cursor::Goto(1, size + 1), p.ip_strings[cursor as usize]).expect("X");
        match ips_hash.get(&p.ip_strings[cursor as usize]) {
            Some(pos) => {
                //for reasons I don't understand, using write! here really
                //reduces responsiveness
                writeln!(stdout,"{}{}", cursor::Goto(pos.x,pos.y),"X").expect("X");
            },
            None => {}
        };
        match prokrx.recv() {
            Ok(result) => {
                match result  {
                    PingResultOrKey::Ping(mpresult) => {
                        let (addr, rtt) = match mpresult { 
                            MyPingResult::Idle{addr}         => { (addr, 5000            as isize) },
                            MyPingResult::Receive{addr, rtt} => { (addr, rtt.as_millis() as isize) }
                        };
                        //write!(stdout,"{}", cursor::Goto(30,1)).expect("X");
                        //write!(stdout,"{}, {}ms                   ",addr.to_string(),rtt).expect("X"); 
                        let pos = match ips_hash.get(&addr.to_string()) {
                            Some(pos) => pos,
                            None => { continue }
                        };
                        write!(stdout,"{}", cursor::Goto(pos.x,pos.y)).expect("X");
                        if      rtt ==5000 { write!(stdout,"{}", s5).expect("X"); } 
                        else if rtt > 2000 { write!(stdout,"{}", s4).expect("X"); } 
                        else if rtt > 1000 { write!(stdout,"{}", s3).expect("X"); } 
                        else if rtt > 100  { write!(stdout,"{}", s2).expect("X"); }
                        else if rtt > 10   { write!(stdout,"{}", s1).expect("X"); }
                        else               { write!(stdout,"{}", s0).expect("X"); }
                    },
                    PingResultOrKey::Key(k) => {
                        write!(stdout,"{}", cursor::Goto(1, size + 1)).expect("X");
                            match k {
                                Key::Char('q') => break 'mainloop,
                                Key::Char('n') => if cursor < (count - 1) as isize { cursor = cursor + 1},
                                Key::Char('p') => if cursor > 0 { cursor = cursor - 1},
                                _ => {}
                            }
                    }
                }
            },
            Err(_) => {},
        }
    }
    writeln!(stdout,"{}", cursor::Show).expect("Could not show cursor");
}
