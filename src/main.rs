use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::*;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::*;

struct Player {
    tag: String,
    sock: TcpStream
}

fn relay(mut red: TcpStream, mut blue: TcpStream) {
    blue.set_nonblocking(true).unwrap();
    red.set_nonblocking(true).unwrap();
    loop {
        let mut red_read = String::new();
        let mut blue_read = String::new();
        {
            let mut red_reader = BufReader::new(&mut red); // We will use is_empty in nightly.
            match red_reader.read_line(&mut red_read) {
                Ok(n) => {
                    if 0 < n {
                        blue.write(red_read.as_bytes()).unwrap();
                    }
                }
                Err(_) => {
                }
            }
        }
        {
            let mut blue_reader = BufReader::new(&mut blue);
            match blue_reader.read_line(&mut blue_read) {
                Ok(n) => {
                    if 0 < n {
                        red.write(blue_read.as_bytes()).unwrap();
                    }
                }
                Err(_) => {
                }
            }
        }
    }
}

fn linker(rx: Receiver<Player>) {
    let mut players = HashMap::new();
    loop {
        let player = rx.recv().unwrap();
        match players.remove(&player.tag) {
            Some(p) => {
                thread::spawn(move|| {
                    relay(p, player.sock);
                });
            }
            None => {
                players.insert(player.tag, player.sock);
            }
        }
        println!("Currently {} players waiting.", players.len());
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1664").expect("Failed to bind port. Is it in use? Do you have neccessarily permission?");
    let (linker_tx, linker_rx) = mpsc::channel();
    thread::spawn(move || {
        linker(linker_rx);
    });
    loop {
        match listener.accept() {
       	    Ok((mut sock, address)) => {
                let linker_tx = linker_tx.clone();
                thread::spawn(move || {
                    let mut tag = String::new();
                    {
                        let mut handle = BufReader::new(&mut sock);
                        handle.read_line(&mut tag).unwrap();
                    }
                    tag = tag.trim().to_owned();
                    println!("New player {} joined lobby '{}'.", address, tag);
                    let player = Player{tag: tag, sock: sock};
                    linker_tx.send(player).unwrap();
                });
	    }
	    Err(e) => {
	        println!("Failed to connect {}.", e);
	    }
        }
    }
}
