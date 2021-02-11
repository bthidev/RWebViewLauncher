#![windows_subsystem = "windows"]
extern crate web_view;

use async_process::{Command, Stdio};
use futures::executor::block_on;
use futures_lite::{io::BufReader, prelude::*};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use web_view::*;

fn main() {
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}
async fn hello_world() -> bool {
    let contents =
        fs::read_to_string("./config.json").expect("Something went wrong reading the file");
    let config: Config = serde_json::from_str(&contents).expect("JSON was not well-formatted");
    let mut output = Command::new(config.app_path)
        .stdout(Stdio::piped())
        .spawn()
        .expect("error run app");
    let mut lines = BufReader::new(output.stdout.take().unwrap()).lines();

    while let Some(line) = lines.next().await {
        let str_out_put = line.expect("error reading line");
        println!("{}", str_out_put);
        let re = Regex::new(
            "https?://[-a-zA-Z0-9@:%._\\+~#=]{1,256}\\b([-a-zA-Z0-9()@:%_\\+.~#?&//=]*)",
        )
        .unwrap();
        if re.is_match(&str_out_put) {
            web_view::builder()
                .title(&config.name)
                .content(Content::Url("http://localhost:5000"))
                .size(800, 600)
                .resizable(true)
                .debug(true)
                .user_data(())
                .invoke_handler(|_webview, _arg| Ok(()))
                .run()
                .unwrap();
            output.kill().expect("cannot stop app");
            return true;
        }
    }
    return true;
}

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    app_path:String,
}
