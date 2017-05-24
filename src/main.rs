mod api;

fn main() {
    let status = api::get_status();

    println!("{:?}", status);

    let launch = api::launch("");

    println!("{:?}", launch);
}
