struct RustySC {
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
}