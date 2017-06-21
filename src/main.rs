mod api;
extern crate time;
extern crate chrono;

use std::io::{self, Write};
use chrono::prelude::*;
use time::{Tm, Duration};
use std::fs::File;
use std::io::prelude::*;

use std::io::BufReader;

fn main() {
    let mut last_launch: Option<Tm> = match File::open("last_launch.txt") {
        Error => None,
        Ok(f) => {
            let mut time = String::new();

            BufReader::new(f).read_to_string(&mut time).unwrap();

            match time::strptime(&time, "%+") {
                Error => None,
                Ok(t) => Some(t)
            }
        }
    };

    println!("Salutations, President. Please insert the password to ahniliate (if that's how you spell it) our enemies!");
    loop {
        // TODO: handle error.
        println!("The system is {}",
                 if api::get_status().unwrap() {
                     "online"
                 } else {
                     "offline"
                 });

        let password = build_password();

        if can_launch(last_launch) {
            let launch_result = api::launch(&password);
            match launch_result {
                Ok(message) => {
                    println!("{}", message);
                    last_launch = Option::Some(time::now_utc());
                    match File::open("last_launch.txt") {
                        Ok(mut f) => {f.write(last_launch.unwrap().rfc3339().to_string().as_bytes());},
                        Err(_) => ()
                    };
                }
                Err(message) => println!("ERROR: {}", message),
            }
        }
    }
}

fn get_password() -> Result<String, std::io::Error> {
    print!("Password: ");
    io::stdout().flush()?;

    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    Result::Ok(password)
}

fn can_launch(last_launch: Option<Tm>) -> bool {
    let launch_cooldown = Duration::minutes(5);

    match last_launch {
        Some(last_launch_time) => {
            let since_last_launch = time::now_utc() - last_launch_time;
            if since_last_launch < launch_cooldown {
                println!("Apologies, estimeed President, the minimum cooldown time of {:?} has not been respected, please wait {:?} more seconds",
                         launch_cooldown,
                         launch_cooldown - since_last_launch);
                return false;
            }
            return true;
        }
        None => return true,
    }
}

fn build_password() -> String {
    let entered_password;
    loop {
        let password_result = get_password();
        match password_result {
            Ok(_) => {
                // TODO: future stefano's problem.
                entered_password = password_result.unwrap();
                break;
            }
            Err(_) => {}
        }
    }

    let current_date = Local::now();
    let date_string  = current_date.format("%y%m%d").to_string();
    let password     = date_string + "-" + &entered_password;

    return password;
}