extern crate hyper;
use self::hyper::Client;

use std;
use std::result::Result;
use std::io::Read;

extern crate json;

pub fn get_status() -> Result<bool, String> {
    let client = Client::new();

    let request_result = client
        .get("http://gitland.azurewebsites.net:80/api/warheads/status")
        .send();

    match request_result {
        Ok(mut response) => {
            let mut api_status_json = String::new();
            let read_to_string_result = response.read_to_string(&mut api_status_json);

            match read_to_string_result {
                Ok(_) => {
                    let parse_result = json::parse(api_status_json.as_str());
                    
                    match parse_result {
                        Ok(parsed) => {
                            let api_status = parsed["Status"] == "Online";
                            return Result::Ok(api_status);
                        }
                        Err(err) => return Err(err.to_string()),
                    }
                }
                Err(err) => return Err(err.to_string()),
            }
        }
        Err(err) => return Result::Err(err.to_string()),
    }
}

#[test]
fn get_status_full() {
    assert_eq!(get_status().is_ok(), true);
}
