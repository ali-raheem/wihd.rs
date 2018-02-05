use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::*;
use std::io::*;
use std::str;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::*;

struct Player {
    tag: String,
    sock: TcpStream
}
struct Pair {
    Red: Player,
    Blue: Player
}

fn relay(mut red: TcpStream, mut blue: TcpStream) {
    blue.set_nonblocking(true).expect("no block");
    red.set_nonblocking(true).expect("no block");
    loop {
        let mut redRead = String::new();
        let mut blueRead = String::new();
        {
            let mut redReader = BufReader::new(&mut red);
            let mut blueReader = BufReader::new(&mut blue);
            redReader.read_line(&mut redRead);
            blueReader.read_line(&mut blueRead);
        }
        blue.write(redRead.as_bytes());
        red.write(blueRead.as_bytes());
    }
}

fn linker(rx: Receiver<Player>) {
    let mut Players = HashMap::new();
    loop {
        let player = rx.recv().unwrap();
        println!("{}", player.tag);
        match Players.remove(&player.tag) {
            Some(p) => {
                thread::spawn(move|| {
                    relay(p, player.sock);
                });
            }
            None => {
                Players.insert(player.tag, player.sock);
            }
        }
        println!("Currently {} players waiting.", Players.len());
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1664").unwrap();
    let (linkerTx, linkerRx) = mpsc::channel();
    thread::spawn(move || {
        linker(linkerRx);
    });
    loop {
        match listener.accept() {
       	    Ok((mut sock, address)) => {
                let lTx = linkerTx.clone();
                thread::spawn(move || {
                    let mut tag = String::new();
                    {
                        let mut handle = BufReader::new(&mut sock);
                        handle.read_line(&mut tag);
                    }
                    tag = tag.trim().to_owned();
                    println!("New Player {:?} joining \"{}\" lobby.", address, tag);
                    let player = Player{tag: tag, sock: sock};
                    lTx.send(player);
                });
	    }
	    Err(e) => {
	        println!("Failed to connect");
	    }
        }
    }
}
