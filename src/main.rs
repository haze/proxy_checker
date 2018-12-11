extern crate regex;
use regex::Regex;


extern crate rayon;
use rayon::prelude::*;

extern crate clap;
use clap::{Arg, App};

extern crate reqwest;

use std::path::Path;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};

fn main() {
    let matches = App::new("proxy checker")
                          .version("1.0")
                          .author("Haze B. <haze@ill.fi>")
                          .about("checks proxies")
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("ip")
                               .short("x")
                               .multiple(false)
                               .help(""))
                          .get_matches();
    let file = matches.value_of("INPUT").unwrap();
    let check_ip = matches.value_of("ip");
    if Path::new(file).exists() {
        let mut proxies: Vec<String> = Vec::new();
        for line in BufReader::new(File::open(file).expect("failed to open file")).lines() {
            proxies.push(line.unwrap());
        }
        let chunks: Vec<&[String]> = proxies.as_slice().chunks(100).collect();
        let re = Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$").unwrap();
        chunks.par_iter().for_each(|proxs| {
            for prox in *proxs {
                let proxy = reqwest::Proxy::http(&*format!("http://{}", prox.clone()));
                if !proxy.is_ok() { return }
                let client = reqwest::Client::builder().proxy(proxy.unwrap()).build().expect("failed to build client");
                let mut ip = String::new();
                let resp = client.get("http://api.ipify.org").send();
                if !resp.is_ok() { return }
                if check_ip.is_some() {
                    resp.unwrap().read_to_string(&mut ip).expect("failed to read back ip");
                    if re.is_match(&*ip) {
                        println!("{}", prox);
                    }
                } else {
                    println!("{}", prox);
                }
            }
        });
    } else {
        println!("file \"{}\" does not exist", file)
    }
}
