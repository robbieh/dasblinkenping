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
//use fastping_rs::PingResult::{Idle, Receive};

use std::cmp;
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


#[derive(Debug, Clone, PartialEq)]
pub enum UiState { Clear, Stats, CommandLine, Auto }

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
            } else {
                let cidr6 = <IpCidr>::from(cidr);
                for ip in cidr6.iter() {
                    println!("cidr ip: {:#?}", ip);
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

fn rtt_sym(symvec: &Vec<char>, rtt: isize) -> char {
    if      rtt ==5000 { symvec[5] }
    else if rtt > 2000 { symvec[4] }
    else if rtt > 1000 { symvec[3] }
    else if rtt > 100  { symvec[2] }
    else if rtt > 10   { symvec[1] }
    else               { symvec[0] }
}

fn get_adjacent(
    point_pos_hash: &HashMap<hilbert::Point,usize>, 
    pos_point_hash: &HashMap<usize,hilbert::Point>,
    current: &usize,
    size: &u16,
    xdelta: i16, 
    ydelta: i16
) 
    -> usize
{
    let pt = match pos_point_hash.get(&current) {
        None => return 0,
        Some(pt) => pt
    };
    let (mut x,mut y) = (pt.x as i16 + xdelta, pt.y as i16 + ydelta);
    x = cmp::max(x,1i16);
    y = cmp::max(y,1i16);
    x = cmp::min(x,*size as i16);
    y = cmp::min(y,*size as i16);
    let newpt = hilbert::Point{x: x as u16, y: y as u16};
    match point_pos_hash.get(&newpt) {
        None => *current,
        Some(pos) => *pos
    }
}

fn main() {
    let p = parse_args();
    let mut cursor = 0 as usize; //cursor position on ip line
    let mut ui = UiState::Clear;

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
    if count == 0 {
        println!("No usuable IPs or CIDRs were supplied.");
        std::process::exit(1);
    }
    let sqrt = (count as f64).sqrt();
    let sqrt_again = (sqrt as f64).sqrt();
    let mut size = sqrt_again.ceil() as u16;
    size = (2.0_f64).powi(size as i32 ) as u16;
    let hilbert_points = hilbert::hilbert(sqrt_again.ceil() as isize);
    let mut ip_point_hash = HashMap::new();
    let mut point_pos_hash = HashMap::new();
    let mut pos_point_hash = HashMap::new();
    let mut point_rtt_hash = HashMap::new();
    for n in 0..count {
        pinger.add_ipaddr(&p.ip_strings[n]);
        ip_point_hash.insert(&p.ip_strings[n], hilbert_points[n].clone());
        point_pos_hash.insert(hilbert_points[n].clone(),n);
        pos_point_hash.insert(n,hilbert_points[n].clone());
        point_rtt_hash.insert(hilbert_points[n].clone(),5000);
    }

    pinger.run_pinger();
    let (proktx, prokrx) = channel();
    let p2= proktx.clone();
    thread::spawn(move || {pinger_thread(p2,results);});

    //set up keyboard input thread
    //let (kbdtx, kbdrx) = channel();
    thread::spawn(move || {keyboard_thread(proktx);});

    //let symbols = "∙∘⊙⊚● ";
    let symbols = "∙∘⊙⊚● ";
    //let symbols = "●⊚⊙∘∙ ";
    //let symbols = "●●∘∘∙ ";
    let symvec: Vec<char> = symbols.chars().collect();

    let mut stdout = stdout().into_raw_mode().unwrap();
    writeln!(stdout,"{}", clear::All).expect("Could not clear screen");
    writeln!(stdout,"{}", cursor::Hide).expect("Could not hide cursor");

    'mainloop: loop{
        if ! running.load(Ordering::SeqCst) { 
            writeln!(stdout,"{}", cursor::Goto(1, size + 1)).expect("X");
            pinger.stop_pinger();
            break; 
        };

        //show cursor and IP
        match ui {
            UiState::Clear => {
                writeln!(stdout,"{}                  ", cursor::Goto(1, size + 1)).expect("X");
            },
            UiState::Stats => {
                writeln!(stdout,"{}IP: {}              ", cursor::Goto(1, size + 1), p.ip_strings[cursor as usize])
                    .expect("X");
                match ip_point_hash.get(&p.ip_strings[cursor as usize]) {
                    Some(point) => {
                        writeln!(stdout,"{}{}{}{}", cursor::Goto(point.x,point.y),
                            style::Invert,
                            rtt_sym(&symvec,*point_rtt_hash.get(point).unwrap()),
                            style::NoInvert,
                            ).expect("X");
                    },
                    None => {}
                };
            },
            _ => {}
        }

        //writeln!(stdout,"{}cursor: {}              ", cursor::Goto(1, size + 2), cursor as usize).expect("X");

        match prokrx.recv() {
            Err(_) => {},
            Ok(result) => {
                match result  {
                    PingResultOrKey::Ping(mpresult) => {
                        let (addr, rtt) = match mpresult { 
                            MyPingResult::Idle{addr}         => { (addr, 5000            as isize) },
                            MyPingResult::Receive{addr, rtt} => { (addr, rtt.as_millis() as isize) }
                        };

                        //ugly debug
                        //writeln!(stdout,"{}", cursor::Goto(30,1)).expect("X");
                        //writeln!(stdout,"{}, {}ms                   ",addr.to_string(),rtt).expect("X"); 
                        //writeln!(stdout,"{}{}", cursor::Goto(30,1),count).expect("X");

                        let pos = match ip_point_hash.get(&addr.to_string()) {
                            Some(pos) => pos,
                            None => { continue }
                        };
                        point_rtt_hash.insert(pos.clone(),rtt);
                        writeln!(stdout,"{}{}", cursor::Goto(pos.x,pos.y),rtt_sym(&symvec,rtt)).expect("X");
                    },
                    PingResultOrKey::Key(k) => {
                        if ui != UiState::Clear {
                            match ip_point_hash.get(&p.ip_strings[cursor as usize]) {
                                None => {},
                                Some(point) => {
                                    //restore symbol under cursor position
                                    match point_rtt_hash.get(point) {
                                        None => {},
                                        Some(rtt) => {
                                            let sym = rtt_sym(&symvec,*rtt);
                                            writeln!(stdout,"{}{}", cursor::Goto(point.x,point.y),sym).expect("X");
                                        }
                                    };
                                }
                            }
                        }
                        match k {
                            Key::Char('q') => break 'mainloop,
                            Key::Esc => { ui = UiState::Clear; }
                            _ => {handle_key(&k,&mut ui,&mut cursor,&size,&count,&point_pos_hash,&pos_point_hash)}
                        }
                    }
                }
            },
        }
    }
    writeln!(stdout,"{}", cursor::Show).expect("Could not show cursor");
}

fn handle_key(
    k: &Key,
    ui: &mut UiState,
    cursor: &mut usize,
    size: &u16,
    count: &usize,
    point_pos_hash: &HashMap<hilbert::Point,usize>, 
    pos_point_hash: &HashMap<usize,hilbert::Point>,
    ) {
    if *ui == UiState::Clear { *ui = UiState::Stats };
    match *k {
        Key::Char('n') => if *cursor < (count - 1) { *cursor = *cursor + 1},
        Key::Char('p') => if *cursor > 0 { *cursor = *cursor - 1},
        Key::Char('h') => {*cursor=get_adjacent(&point_pos_hash,&pos_point_hash,&cursor,&size,-1,0)},
        Key::Char('j') => {*cursor=get_adjacent(&point_pos_hash,&pos_point_hash,&cursor,&size,0,1)},
        Key::Char('k') => {*cursor=get_adjacent(&point_pos_hash,&pos_point_hash,&cursor,&size,0,-1)},
        Key::Char('l') => {*cursor=get_adjacent(&point_pos_hash,&pos_point_hash,&cursor,&size,1,0)},
        _ => {}
    }
}
