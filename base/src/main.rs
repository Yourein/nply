use dotenv;

fn main() {
    dotenv::dotenv().ok();

    println!{"{}", dotenv::var("TEST_VAR").unwrap()};
}
