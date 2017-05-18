extern crate hyper;
use self::hyper::Client;

use std::result::Result;
use std::io::Read;

extern crate json;

macro_rules! try_err_string {
    ($x:expr) => (try!($x.map_err(|err| err.to_string())))
}

fn http_get_data(address: &str) -> Result<String, String> {
    // Make call
    let http_client = Client::new();
    let request_result = http_client.get(address).send();

    let mut http_response = try_err_string!(request_result);

    // Read response string
    let mut response_data = String::new();
    let read_to_string_result = http_response.read_to_string(&mut response_data);

    if !read_to_string_result.is_ok() {
        return Err(read_to_string_result.unwrap_err().to_string());
    }

    return Ok(response_data);
}

pub fn get_status() -> Result<bool, String> {
    let status_data_result = http_get_data("http://gitland.azurewebsites.net:80/api/warheads/status");

    let api_status_json = try!(status_data_result);

    // Parse response
    let parse_result = json::parse(api_status_json.as_str());
    let parsed = try_err_string!(parse_result);

    // Determine status
    let api_status = parsed["Status"] == "Online";

    return Result::Ok(api_status);
}

#[test]
fn get_status_full() {
    assert_eq!(get_status().is_ok(), true);
}
