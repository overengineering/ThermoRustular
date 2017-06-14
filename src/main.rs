mod api;
extern crate time;

use std::io::{self, Write};
use time::{Tm,Duration};

fn main() {
    let mut last_launch: Option<Tm> = None;
    let launch_cooldown = Duration::minutes(5);
    println!("Salutations, President. Please insert the password to ahniliate (if that's how you spell it) our enemies!");

    loop{
        // TODO: handle error.
        println!("The system is {}", if api::get_status().unwrap() {"online"} else {"offline"});

        let password;
        loop{
            let password_result = get_password();
            match password_result {
                Ok(_) => {
                    // TODO: future stefano's problem.
                    password = password_result.unwrap();
                    break;
                },
                Err(_) => {},
            }
        }

        match last_launch {
            Some(last_launch_time) => {
                let since_last_launch = time::now_utc() - last_launch_time;
                if since_last_launch < launch_cooldown {
                    println!(
                        "Apologies, estimeed President, the minimum cooldown time of {:?} has not been respected, please wait {:?} more seconds",
                        launch_cooldown,
                        launch_cooldown - since_last_launch
                    );
                    continue;
                }
            },
            None => {},
        }
        
        let launch_result = api::launch(&password);
        match launch_result {
            Ok(message) => {
                println!("{}", message);                
                last_launch = Option::Some(time::now_utc());
            },
            Err(message) => println!("ERROR: {}", message),
        }
    }    
}

fn get_password() -> Result<String, std::io::Error>{
    print!("Password: ");
    io::stdout().flush()?;

    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    Result::Ok(password)
}