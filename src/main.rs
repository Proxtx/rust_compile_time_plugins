include!(concat!(env!("OUT_DIR"), "/plugins.rs"));

pub trait Plugin {
    fn init(&self);
}

fn main() {
    let t = Plugins::init();
    t.plugins[0].init();
}
