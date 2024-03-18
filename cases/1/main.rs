pub struct Console {}

impl Console {
    pub fn log(value: &str) {
        println!("{}", value);
    }
}

const console: Console = Console {};

fn main() {
    console.log("Hello World!");
}
