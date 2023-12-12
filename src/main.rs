use clap::Parser;
use std::time::Duration;
use reqwest::StatusCode;


#[derive(Debug, Parser)]
struct Args {
    interval: u64,
    url: String,
}

fn print_status(url: &str, status: &str) {
    println!("Checking '{url}'. Result: {status}", )
}

fn main() {
    let args = Args::parse();

    loop {
        let resp_result = reqwest::blocking::get(&args.url);
        if let Err(err) = &resp_result {
            if err.is_builder() {
                print_status(&args.url, "URL parsing error");
            } else {
                print_status(&args.url, &format!("ERR({:?})", err));
            }
            return;
        }
        let resp = resp_result.unwrap();

        match resp.status() {
            StatusCode::OK => print_status(&args.url, "OK(200)"),
            _ => print_status(&args.url, &format!("ERR({})", resp.status().as_u16()))
        }

        std::thread::sleep(Duration::from_secs(args.interval))
    }
}
