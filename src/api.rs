extern crate hyper;
use self::hyper::Client;
use std::io::Read;

pub fn get_status() -> bool {
    let client = Client::new();

    let mut res = client.get("http://gitland.azurewebsites.net:80/api/warheads/status")
        .send()
        .unwrap();

    let mut api_status = String::new();
    let result = res.read_to_string(&mut api_status);
    
    return api_status == "Online";
}
