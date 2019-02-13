extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("plsget")
        .version("0.1.0")
        .author("Advaith Doosa <advaith@doosa.org>")
        .about("pls get http requests")
        .arg(Arg::with_name("URL")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("url to download"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("{}", url);
}
