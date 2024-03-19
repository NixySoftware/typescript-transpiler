pub struct Console {}

impl Console {
    pub fn assert(&self, assertion: bool) {
        assert!(assertion);
    }

    pub fn log(&self, value: &str) {
        println!("{}", value);
    }
}

#[allow(non_upper_case_globals)]
pub const console: Console = Console {};
