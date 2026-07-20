pub mod practice01;
mod practice_cli_app;
mod file_contents_query;
mod close_package;

fn main() {
    println!("========== practice demo code ==========");
    // practice01::run();
    // practice_cli_app::run();
    file_contents_query::start();
    // close_package::run();


}

#[test]
fn my_rust_test() {
    println!("This is a test function");
}
