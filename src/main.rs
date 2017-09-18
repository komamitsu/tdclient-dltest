extern crate td_client;
use td_client::client::*;
use std::env;
use std::fs::File;

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let apikey = args.next().unwrap();
    let jobid = args.next().unwrap();
    let client = Client::new(apikey.as_str());

    let result_file = File::create("/dev/null").unwrap();
    client.download_job_result(jobid.parse::<u64>().unwrap(), &result_file).unwrap();

    println!("finish");
}

