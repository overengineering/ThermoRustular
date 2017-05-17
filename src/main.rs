use std::io::{self, Write};

fn main() {
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

        
    }    
}

fn get_password() -> Result<String, std::io::Error>{
    print!("Password: ");
    io::stdout().flush()?;

    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    Result::Ok(password)
}