extern crate json;
extern crate hyper; 

macro_rules! try_err_string {
    ($x:expr) => (try!($x.map_err(|err| err.to_string())))
}

fn http_get_data(address: &str) -> Result<String, String> {
    extern crate hyper;

    // Make call
    let http_client = hyper::Client::new();
    let request_result = http_client.get(address).send();

    return get_string_from_http_call(request_result);
}

fn http_post_data(address: &str, data: &str) -> Result<String, String> {
    // Make call
    let http_client = hyper::Client::new();
    let request_result = http_client.post(&(address.to_owned() + "?" + data))
        .send();

    return get_string_from_http_call(request_result);
}

fn get_string_from_http_call(request_result: hyper::Result<hyper::client::Response>) -> Result<String, String> {
    use std::io::Read;

    let mut http_response = try_err_string!(request_result);

    // Read response string
    let mut response_data = String::new();

    try_err_string!(http_response.read_to_string(&mut response_data));

    return Ok(response_data);
}


// ---


pub fn get_status() -> Result<bool, String> {
    // Get data
    let status_data_result = http_get_data(
        "http://gitland.azurewebsites.net:80/api/warheads/status");

    let api_status_json = try!(status_data_result);

    // Parse response
    let parse_result = json::parse(api_status_json.as_str());
    let parsed = try_err_string!(parse_result);

    // Determine status
    let api_status = parsed["Status"] == "Online";

    return Result::Ok(api_status);
}

pub fn launch(launch_code: &str) -> Result<String, String> {
    // Get data
    let launch_data_result = http_post_data(
        "http://gitland.azurewebsites.net:80/api/warheads/launch",
        ("launchCode=".to_owned() + launch_code).as_str());

    let launch_json = try!(launch_data_result);

    // Parse response
    let parse_result = json::parse(launch_json.as_str());
    let parsed = try_err_string!(parse_result);

    // Determine status
    let is_success = parsed["Result"] == "Success";
    let message = parsed["Message"].as_str();

    if is_success {
        return Result::Ok(message.unwrap().to_string());
    } else {
        return Result::Err(message.unwrap().to_string());
    }
}


// ---


#[test]
fn get_status_full() {
    assert_eq!(get_status().is_ok(), true);
}

#[test]
fn launch_full_no_code() {
    assert_eq!(launch("").is_ok(), false);
}