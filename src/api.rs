extern crate hyper;
use self::hyper::Client;

use std;
use std::result::Result;
use std::io::Read;

extern crate json;

pub fn get_status() -> Result<bool, String> {
    // Make call
    let http_client = Client::new();
    let request_result =  http_client.get("http://gitland.azurewebsites.net:80/api/warheads/status")
        .send();

    let mut response = try!(request_result.map_err(|err| err.to_string()));

    // Read response
    let mut api_status_json = String::new();
    let read_to_string_result = response.read_to_string(&mut api_status_json);

    if !read_to_string_result.is_ok() {
        return Err(read_to_string_result.unwrap_err().to_string());
    }

    // Parse response
    let parse_result = json::parse(api_status_json.as_str());
    let parsed = try!(parse_result.map_err(|err| err.to_string()));

    // Determine status
    let api_status = parsed["Status"] == "Online";
    
    return Result::Ok(api_status);
}

#[test]
fn get_status_full() {
    assert_eq!(get_status().is_ok(), true);
}
