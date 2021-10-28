// use eyre::{ErrReport, Result};
use chrono::prelude::*;
use eyre::Result;
use reqwest::Url;
use serde::Deserialize;
use serde_json;
use std::env;
// use std::path::Iter;
// use serde_json::{json, Map, Value};
// use std::ops::Index;
use std::time::Duration;

fn process_line(input: String) -> Result<(), String> {
    // println!("we got this input: {}", &input);
    let mut iter = input.split(' ');

    match iter.next() {
        Some("get") => {
            let mut port:Option<u16> = Some(8081);

            let try_next = iter.next();
            let try_port = iter.next();

            if try_next.is_some() && try_port.is_some() {
                if try_next.unwrap() == "-p" || try_next.unwrap() == "--port" {
                    if !try_port.is_none() {
                        if !try_port.unwrap().parse::<u16>().is_err() {
                            port = Some(try_port.unwrap().parse::<u16>().unwrap());
                            println!("process_line: doing request on port number {}", port.unwrap());
                        } else {
                            return Err("bad port number".to_string());
                        }
                    } else {
                        return Err("missing port number".to_string());
                    }
                }
            }


            let mut next = Url::parse(iter.next().unwrap_or("https://marf.xyz/archive?test"));
            if next.is_err() {
                return Err(next.unwrap_err().to_string());
            }
            let _ = next.as_mut().expect("Bad URL").set_port(port);
            let client = reqwest::blocking::Client::new();
            let request = client
                .get(next.unwrap())
                .timeout(Duration::from_secs(4))
                .send();
            let result:String;
            if !request.is_err() {
                result = request.unwrap().text().unwrap_or("bad result".into());
            } else {
                return Err(request.unwrap_err().to_string());
            }
            println!("process_line: got this, {}", result);

            let mut json = serde_json::Deserializer::from_str(result.as_str());
            // THIS crashes on an empty string, investigate and patch
            let value;
            match serde_json::Value::deserialize(&mut json) {
                Err(msg) => return Err(msg.to_string()),
                Ok(processed) => value = processed,
            }
            if value.get("now").is_none() {
                return Err("Key [now] not found in JSON, can't continue".to_string());
            }
            if value.get("now").unwrap().as_f64().is_none() {
                return Err("Can't parse JSON [now] as f64, invalid number??".to_string());
            }
            let naive = NaiveDateTime::from_timestamp(
                value
                    .get("now")
                    .unwrap()
                    .as_f64()
                    .expect("No date or bad provided") as i64,
                0,
            );
            let date: DateTime<Utc> = DateTime::from_utc(naive, Utc);
            println!("right now it is: {} time", date);
            println!("I've managed to parse the following JSON:");
            for map in value.as_object() {
                for key in map.keys() {
                    if key == "testing" {
                        println!("testing is: {}", value.get(key).unwrap());
                    }
                    match value.get(key) {
                        Some(val) => println!("{}: {}", key, val),
                        None => {}
                    }
                }
            }

            /*match value.as_object() {
                Some(map) => {
                    if !map.contains_key("testing") {
                        return Err("Key doesn't exist");
                    }
                }
                None => {}
            }*/
            // println!("testing is: {}", value.get("testing").unwrap());
        }
        line => {
            println!("Invalid option <{}>.", line.unwrap_or("Invalid input"));
        }
    };

    Ok(())
}

fn main() -> Result<()> {
    init()?;

    let mut prompt = env::var("PROMPT").unwrap();
    if prompt.is_empty() {
        println!("no prompt");
        prompt = "> ".into();
    }

    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(line) = rl.readline(&prompt) {
        match line.trim().split(' ').next() {
            Some("exit") | Some("quit") => break,
            Some(_) => {
                let result = process_line(line);
                if result.as_ref().is_err() {
                    let _ = result.map_err(|err| println!("[ERR] {}", err));
                };
            }
            None => {
                println!("Invalid...? bruh rust");
            }
        }
    }

    Ok(())
}

fn init() -> Result<()> {
    dotenv::dotenv()?;
    color_eyre::install()?;

    Ok(())
}
