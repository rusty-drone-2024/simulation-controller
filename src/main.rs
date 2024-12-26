struct MySimulationController {
    network: Network,
}

impl MySimulationController {
    fn start(network: Network) {
        let controller = MySimulationController { network };
        controller.run();
    }

    fn run(&self) {
        println!("Running simulation controller");
    }
}