extern crate clap;
extern crate rusoto_core;
extern crate rusoto_route53;
extern crate env_logger;
extern crate reqwest;

use clap::{Arg, App};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};
use rusoto_route53::{Route53Client, Route53, ListHostedZonesRequest};
use std::io::Read;

fn main() {

    let _ = env_logger::init();

    let matches = App::new("Route53 updater for EC2")
                        .version("1.0")
                        .author("Chamindu R. Munasinghe <cmunasinghe@gmail.com>")
                        .about("This tool updates route 53 A records whith the public IP of your EC2 instance.")
                        .arg(Arg::with_name("hostedzone")
                             .short("z")
                             .long("zone")
                             .help("The route53 hosted zone id")
                             .required(true)
                             .takes_value(true))
                        .arg(Arg::with_name("name")
                             .short("n")
                             .long("name")
                             .help("The dns name for the A record")
                             .required(true)
                             .takes_value(true))
                        .get_matches();

    println!("hosted zone {}, dns name {}", matches.value_of("hostedzone").unwrap(), matches.value_of("name").unwrap());

    //let provider = DefaultCredentialsProvider::new().unwrap();
    //let client = Route53Client::new(default_tls_client().unwrap(), provider, Region::UsEast1);

    
    let mut response = reqwest::get("http://169.254.169.254/latest/meta-data/public-ipv4")
        .expect("Unable to retrieve the public IP address from metadata. Are we running inside an EC2 instance?");

    let mut content = String::new();

    response.read_to_string(&mut content);

    println!("{}", content);

}
