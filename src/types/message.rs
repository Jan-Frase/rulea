
#[repr(C)]
pub struct Message {
    lang_name: String,
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}

impl Message {
    pub fn new() -> Self {
        Self{
            lang_name: "Rust".parse().unwrap(),
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn print_hello(&self) {
        println!("Hello from {}!", self.lang_name)
    }
}