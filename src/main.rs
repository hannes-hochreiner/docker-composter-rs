use std::error::Error;
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("docker-composter-rs")
                    .version("0.1.0")
                    .author("Hannes Hochreiner <hannes@hochreiner.net>")
                    .about("Provides a fertile ground for your application to grow.")
                    .arg(Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("sets a custom configuration file")
                        .takes_value(true)
                    )
                    .subcommand(SubCommand::with_name("up"))
                    .subcommand(SubCommand::with_name("down"))
                    .get_matches();

    let config = matches.value_of("config").unwrap();
    println!("Value for config: {}", config);
    
    match matches.subcommand() {
        ("up", _) => {
            println!("up");
        },
        ("down", _) => {
            println!("down");
        },
        (&_, _) => {
            panic!("unknown sub-command");
        }
    }

    Ok(())
}
