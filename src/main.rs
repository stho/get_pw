
extern crate keyring;

use std::io;
use std::error::Error;
use std::env;
use std::process;

// fn main() -> Result<(), Box<dyn Error>> {
fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // let parmeter1 = &args[2];

    println!("cmd: {}", config.cmd);
    println!("subcmd: {}", config.subcmd);


    // let service = "mount-ind-shares";
    // let username = "hh";

    // let keyring = keyring::Keyring::new(&service, &username);
    // println!("Please enter a new password");

    // let mut pwd = String::new();

    // io::stdin().read_line(&mut pwd)
    //     .expect("Failed to read line");
    // keyring.set_password(&pwd)?;

    // let password = keyring.get_password()?;
    // println!("The password is '{}'", password);

    // Ok(())
}

struct Config {
    cmd: String,
    subcmd: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let cmd = args[1].clone();
        let subcmd = args[2].clone();

        Ok(Config { cmd, subcmd })
    }
}
