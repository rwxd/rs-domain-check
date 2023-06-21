use std::fs::OpenOptions;
use std::io::prelude::*;

use whois_rust::{WhoIs, WhoIsLookupOptions};
use std::{thread, time::Duration};


fn main() {
    let whois = WhoIs::from_path("src/servers.json").unwrap();
    let path = "src/german-words.txt";
    let todo = get_words();
    println!("=> Loaded {} domains to check", todo.len());
    let mut done = get_done();
    println!("=> Loaded {} already checked domains", done.len());
    let domains_left: Vec<&String> = todo.iter().filter(|domain| !done.contains(&domain.to_string())).collect();
    println!("=> {} domains left to check", domains_left.len());
    for word in todo.iter() {
        // check if word ends with .de
        // if not, add .de
        let domain = match word.ends_with(".de") {
            true => word.to_string(),
            false => {
                format!("{}.de", word)
            }
        };
        if !done.contains(&domain.to_string()) {
            println!("Checking \"{}\"", domain);
            let result = whois.lookup(WhoIsLookupOptions::from_string(domain.to_string()).unwrap());
            let whois_answer = match result {
                Ok(whois_answer) => whois_answer,
                Err(e) => {
                    println!("Error: {}", e);
                    if e.to_string().contains("Connection reset by peer") {
                        println!("=> Connection reset by peer, sleeping for 10 seconds");
                        thread::sleep(Duration::from_millis(10*1000));
                    }
                    continue;
                }
            };
            if whois_answer.contains("Status: free") {
                println!("=> \"{}\" is available", domain);
                write_available(domain.to_string());
            }

            if whois_answer.contains("access control limit exceeded") {
                println!("=> Access control limit exceeded, sleeping for 10 seconds");
                thread::sleep(Duration::from_millis(10*1000));
                continue
            }
            done.push(domain.to_string());
            write_done(domain.to_string());
            thread::sleep(Duration::from_millis(200));
        }
    }
}

fn get_words() -> Vec<String> {
    let mut words = Vec::new();
    let file = OpenOptions::new().read(true).open("src/german-words.txt").unwrap();
    for line in std::io::BufReader::new(file).lines() {
        words.push(line.unwrap());
    }
    words
}

fn get_done() -> Vec<String> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("done.txt")
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n").map(|s| s.to_string()).collect::<Vec<String>>()
}

fn write_done(domain: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("done.txt")
        .unwrap();
    if let Err(e) = writeln!(file, "{}", domain) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn write_available(domain: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("available.txt")
        .unwrap();
    if let Err(e) = writeln!(file, "{}", domain) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
