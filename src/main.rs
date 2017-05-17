mod api;

fn main() {
    let status = api::get_status();

    println!("{:?}", status);
}
