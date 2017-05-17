use std::io::{self, Write};

fn main() {
    println!("Salutations, President. Please insert the password to ahniliate (if that's how you spell it) our enemies!");

    while{
        let password = get_password();
        println!("{}", password.unwrap());
    }    
}

fn get_password() -> Result<String, std::io::Error>{
    print!("Password: ");
    let flush_result = io::stdout().flush();
    match flush_result{
        Err(error)=> return Result::Err(error),
        Ok(_)=>{}
    }

    let mut password = String::new();
    match io::stdin().read_line(&mut password){
        Ok(_) => Result::Ok(password),
        Err(err) => Result::Err(err)
    }
}