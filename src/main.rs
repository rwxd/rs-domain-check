use std::fs::OpenOptions;
use std::io::prelude::*;

use whois_rust::{WhoIs, WhoIsLookupOptions};
use std::{thread, time::Duration};


fn main() {
    let whois = WhoIs::from_path("src/servers.json").unwrap();
    let todo = include_str!("top50k-german-words.txt").split("\n").collect::<Vec<&str>>();
    println!("==> Loaded {} domains to check", todo.len());
    let mut done = get_done();
    println!("==> Loaded {} already checked domains", done.len());
    let domains_left: Vec<&&str> = todo.iter().filter(|domain| !done.contains(&domain.to_string())).collect();
    println!("==> {} domains left to check", domains_left.len());
    for domain in todo.iter() {
        if !done.contains(&domain.to_string()) {
            println!("Checking \"{}\"", domain);
            let result = whois.lookup(WhoIsLookupOptions::from_string(domain.to_string()).unwrap());
            let whois_answer = match result {
                Ok(whois_answer) => whois_answer,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };
            // print!("{}", whois_answer);
            if whois_answer.contains("Status: free") {
                println!("==> \"{}\" is available", domain);
                write_available(domain.to_string());
            }

            if whois_answer.contains("access control limit exceeded") {
                println!("Sleeping for 10 seconds");
                thread::sleep(Duration::from_millis(10*1000));
                continue
            }
            done.push(domain.to_string());
            write_done(domain.to_string());
            thread::sleep(Duration::from_millis(200));
        }
    }
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
