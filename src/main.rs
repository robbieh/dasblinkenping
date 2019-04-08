#[allow(dead_code)]
#[allow(unused)]

#[macro_use] extern crate lazy_static;

extern crate cidr;
extern crate ctrlc;
extern crate lsystem;
extern crate rand;
extern crate regex;
extern crate termion;

mod hilbert;

use cidr::Cidr;
use cidr::IpCidr;
use fastping_rs::Pinger;
use fastping_rs::PingResult::{Idle, Receive};

use std::collections::{HashMap};
use std::env;
use std::io::{Read, Write, stdout};
use std::str::FromStr;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool,Ordering};
use std::time::Duration;


use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, cursor, style};

#[derive(Debug, Clone)]
pub struct Params {
    ip_strings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GuiState {
    highlight: bool,
    current: isize,
    msgs: Vec<GuiMsg>
}

#[derive(Debug, Clone)]
pub enum GuiMsg {
    ShowStats,
    ClearStats,
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
        //if arg.contains("/") { expand_ip_slash(&arg, &mut p); }

    };
    p
}
             
fn main() {
    let mut gs = GuiState{ highlight: false, current: 1, msgs: Vec::new()};
    let p = parse_args();
    println!("params: {:#?}", p);

    //ctrlc
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let count = p.ip_strings.len();
    println!("count: {:#?}", count);
    let sqrt = (count as f64).sqrt();
    println!("sqrt: {:#?}", sqrt);
    let sqrt_again = (sqrt as f64).sqrt();
    println!("sqrt_again: {:#?}", sqrt_again);
    let mut size = sqrt_again.ceil() as u16;
    size = (2.0_f64).powi(size as i32 ) as u16;
    println!("size: {:#?}", size);
    let hilbert_points = hilbert::hilbert(sqrt_again.ceil() as isize);
    let mut ips_hash = HashMap::new();
    let (pinger, results) = match Pinger::new(Some(10000),None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}",e)
    };
        //println!("{:?},{:?}",x,y);
    for n in 0..count {
        pinger.add_ipaddr(&p.ip_strings[n]);
        ips_hash.insert(&p.ip_strings[n], hilbert_points[n].clone());
    }
    pinger.run_pinger();

    //let symbols = "∙∘⊙⊚● ";
      let symbols = "∙∘⊙⊚● ";
    //let symbols = "●⊚⊙∘∙ ";
    //let symbols = "●●∘∘∙ ";
    let s5 = symbols.chars().nth(5).unwrap();
    let s4 = symbols.chars().nth(4).unwrap();
    let s3 = symbols.chars().nth(3).unwrap();
    let s2 = symbols.chars().nth(2).unwrap();
    let s1 = symbols.chars().nth(1).unwrap();
    let s0 = symbols.chars().nth(0).unwrap();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    writeln!(stdout,"{}", clear::All).expect("Could not clear screen");
    writeln!(stdout,"{}", cursor::Hide).expect("Could not hide cursor");
    'mainloop: loop{
        if ! running.load(Ordering::SeqCst) { 
            writeln!(stdout,"{}", cursor::Goto(1, size + 1)).expect("X");
            pinger.stop_pinger();
            break; 
        };
        writeln!(stdout,"{}{}", cursor::Goto(1, size + 1), p.ip_strings[gs.current as usize]).expect("X");
        match ips_hash.get(&p.ip_strings[gs.current as usize]) {
            Some(pos) => {
                writeln!(stdout,"{}{}", cursor::Goto(pos.x,pos.y),"X").expect("X");
            },
            None => {}
        };
        match results.recv_timeout(Duration::from_millis(100)) {
            Ok(result) => {
                let (addr, rtt) = match result { 
                    Idle{addr}         => { (addr, 5000            as isize) },
                    Receive{addr, rtt} => { (addr, rtt.as_millis() as isize) }
                };
                let pos = match ips_hash.get(&addr.to_string()) {
                    Some(pos) => pos,
                    None => { continue }
                };
                writeln!(stdout,"{}", cursor::Goto(pos.x,pos.y)).expect("X");
                if      rtt ==5000 { writeln!(stdout,"{}", s5).expect("X"); } 
                else if rtt > 2000 { writeln!(stdout,"{}", s4).expect("X"); } 
                else if rtt > 1000 { writeln!(stdout,"{}", s3).expect("X"); } 
                else if rtt > 100  { writeln!(stdout,"{}", s2).expect("X"); }
                else if rtt > 10   { writeln!(stdout,"{}", s1).expect("X"); }
                else               { writeln!(stdout,"{}", s0).expect("X"); }
            },
            Err(_) => {},
        }
        writeln!(stdout,"{}", cursor::Goto(1, size + 1)).expect("X");
        for c in stdin.next() {
            match c.unwrap() {
                b'q' => break 'mainloop,
                b'+' => if gs.current < count as isize { gs.current = gs.current + 1},
                b'-' => if gs.current > 1 { gs.current = gs.current - 1},
                _ => {}
            }
        }
    }
    writeln!(stdout,"{}", cursor::Show).expect("Could not hide cursor");
}
