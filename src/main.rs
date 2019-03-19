#[allow(dead_code)]
#[allow(unused)]

extern crate ctrlc;
extern crate termion;
extern crate rand;

use fastping_rs::Pinger;
use fastping_rs::PingResult::{Idle, Receive};

use std::collections::{HashMap};
use std::io::{Read, Write, stdout};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool,Ordering};

use termion::event::{Event,Key};
//use termion::input::TermRead;
use termion::raw::IntoRawMode;
//use termion::screen::AlternateScreen;
use termion::{async_stdin, clear, cursor};
             
fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let size = 16;
    let mut ips_hash = HashMap::new();
    let (pinger, results) = match Pinger::new(None,None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}",e)
    };
    for n in 0..(size * size) {
        let addr_str = IpAddr::V4(Ipv4Addr::new(192,168,2,n as u8)).to_string();
        pinger.add_ipaddr(&addr_str);
        ips_hash.insert(addr_str, n as isize);
    }
    pinger.run_pinger();

    let symbols = "⋅∘○◎● ";
    let s5 = symbols.chars().nth(5).unwrap();
    let s4 = symbols.chars().nth(4).unwrap();
    let s3 = symbols.chars().nth(3).unwrap();
    let s2 = symbols.chars().nth(2).unwrap();
    let s1 = symbols.chars().nth(1).unwrap();
    let s0 = symbols.chars().nth(0).unwrap();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();
    writeln!(stdout,"{}", clear::All).expect("Could not clear screen");
    writeln!(stdout,"{}", cursor::Hide).expect("Cout not hide cursor");
    'mainloop: loop{
        if ! running.load(Ordering::SeqCst) { 
            writeln!(stdout,"{}", cursor::Goto(1, size + 1)).expect("X");
            pinger.stop_pinger();
            break; 
        };
        match results.recv() {
            Ok(result) => {
                let (addr, rtt) = match result { 
                    Idle{addr}         => { (addr, 5000            as isize) },
                    Receive{addr, rtt} => { (addr, rtt.as_millis() as isize) }
                };
                let pos = match ips_hash.get(&addr.to_string()) {
                    Some(pos) => *pos as isize,
                    None => { continue }
                };
                writeln!(stdout,"{}", cursor::Goto((pos % size as isize) as u16 + 1, (pos as f32 /
                                                                             size as f32)
                                                   as u16 + 1) ).expect("X");
                //println!("loop-{:?}, {:?}, {:?}", n, ips[n].addr, pd);
                if      rtt ==5000 { writeln!(stdout,"{}", s5).expect("X"); } 
                else if rtt > 50  { writeln!(stdout,"{}", s4).expect("X"); } 
                else if rtt > 25  { writeln!(stdout,"{}", s3).expect("X"); } 
                else if rtt > 10  { writeln!(stdout,"{}", s2).expect("X"); }
                else if rtt > 5   { writeln!(stdout,"{}", s1).expect("X"); }
                else             { writeln!(stdout,"{}", s0).expect("X"); }
            },
            Err(_) => panic!("Could not run pinger"),
        }
        writeln!(stdout,"{}", cursor::Goto(1, size + 1)).expect("X");
        for c in stdin.next() {
            match c.unwrap() {
                b'q' => break 'mainloop,
                _ => {}
            }
        }
    }
    writeln!(stdout,"{}", cursor::Show).expect("Cout not hide cursor");
}
