extern crate td_client;
use td_client::client::*;
use std::env;

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let apikey = args.next().unwrap();
    let jobid = args.next().unwrap();
    let client = Client::new(apikey.as_str());

    client.each_row_in_job_result(jobid.parse::<u64>().unwrap(), &|xs| {
        let mut s = String::new();
        for x in xs {
            s.push_str(format!("{:?}", x).as_str());
        }
        true
    }).unwrap();

    println!("finish");
}

