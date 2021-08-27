#![windows_subsystem = "windows"]
extern crate web_view;
use std::path::Path;
use async_process::{Command, Stdio};
use futures::executor::block_on;
use futures_lite::{io::BufReader, prelude::*};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use web_view::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = Path::new(&args[0]);
    let rawname = filename.parent().unwrap().to_str().unwrap();
    println!("{}", rawname);
    env::set_current_dir( rawname).expect("Folder Data doesn't exist");
    let future = main_async(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}
fn get_config() -> Config {
    let contents =
        fs::read_to_string("./config.json").expect("Something went wrong reading the file");
    return serde_json::from_str(&contents).expect("JSON was not well-formatted");
}
async fn main_async() -> bool {
    env::set_current_dir("./Data").expect("Folder Data doesn't exist");
    let config = get_config();
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
            let result = re.captures(&str_out_put).unwrap();
            web_view::builder()
                .title(&config.name)
                .content(Content::Url(result.get(0).unwrap().as_str()))
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
    app_path: String,
}
