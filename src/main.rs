extern crate time;

use std::io::{self, Write};
use time::{Tm,Duration};

fn main() {
    let mut last_launch: Option<Tm> = None;
    let launch_cooldown = Duration::minutes(5);
    println!("Salutations, President. Please insert the password to ahniliate (if that's how you spell it) our enemies!");

    loop{
        let mut password;
        loop{
            password = get_password();
            match password {
                Ok(_) => {break;},
                Err(_) => {},
            }
        }

        match last_launch {
            Some(last_launch_time) => {
                let since_last_launch = last_launch_time - time::now_utc();
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
        
        last_launch = Option::Some(time::now_utc());
    }    
}

fn get_password() -> Result<String, std::io::Error>{
    print!("Password: ");
    io::stdout().flush()?;

    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    Result::Ok(password)
}