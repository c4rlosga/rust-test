use eyre::{ErrReport, Result};
use serde_json;
use std::env;
// use std::path::Iter;
use chrono::prelude::*;
use reqwest::Url;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::ops::Index;
use std::time::Duration;

fn process_line(input: String) -> Result<()> {
    // println!("we got this input: {}", &input);
    let mut iter = input.split(' ');
    let _ = match iter.next() {
        Some("get") => {
            let mut next = Url::parse(iter.next().unwrap_or("http://marf.xyz/archive?test"));
            // let _ = next.as_mut().unwrap().set_port(Some(8081));
            let client = reqwest::blocking::Client::new();
            let result = client
                .get(next.unwrap())
                .timeout(Duration::from_secs(4))
                .send()?
                .text()?;
            println!("process_line: got this,\n{}", result);

            let mut json = serde_json::Deserializer::from_str(result.as_str());
            let value = serde_json::Value::deserialize(&mut json).expect("Received bad JSON");
            // let naive = NaiveDateTime::from_timestamp(
            //     value
            //         .get("now")
            //         .expect("No or bad date provided")
            //         .as_f64()
            //         .expect("No date or bad provided") as i64,
            //     0,
            // );
            // let date: DateTime<Utc> = DateTime::from_utc(naive, Utc);
            // let date: DateTime<Utc> = DateTime::from_utc(naive, Utc);
            // println!("right now it is: {} UTC time", date);
            println!("I've managed to parse the following JSON:");
            for map in value.as_object() {
                for key in map.keys() {
                    match value.get(key) {
                        Some(val) => println!("{}: {}", key, val),
                        None => {}
                    }
                }
            }

            match value.as_object() {
                Some(map) => {
                    if !map.contains_key("testing") {
                        return Err("bad");
                    }
                }
                None => {}
            }
            println!("testing is: {}", value.get("testing").unwrap());
        }
        line => {
            println!("Invalid option <{}>.", line.unwrap_or("Invalid input"));
        }
    };

    Ok(())
}

fn main() -> Result<()> {
    init()?;

    let prompt = env::var("PROMPT").unwrap();

    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(line) = rl.readline(&prompt) {
        match line.trim().split(' ').next() {
            Some("exit") | Some("quit") => break,
            Some(_) => {
                let result = process_line(line);
                if result.is_err() {
                    let _ = result.map_err(|err| println!("doodoo: {}", err));
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
