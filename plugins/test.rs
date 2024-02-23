pub struct Plugin {}

impl Plugin {
    pub fn new() -> Self {
        Plugin {}
    }
}

impl crate::Plugin for Plugin {
    fn init(&self) {
        println!("Hello");
    }
}
