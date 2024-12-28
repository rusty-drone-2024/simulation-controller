use common_structs::network::Network;

#[allow(dead_code)]
pub struct RustySC {
    network: Network,
}

impl RustySC {
    pub fn start(network: Network) {
        let controller = RustySC { network };
        controller.run();
    }
}

impl RustySC {
    fn run(&self) {
        println!("Running simulation controller");
    }

    fn add (&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
