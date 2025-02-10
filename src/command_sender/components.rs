use crate::components::Drone;
use wg_2024::controller::DroneCommand;

impl Drone {
    pub fn set_packet_drop_rate(&mut self, pdr: f32) -> Result<(), String> {
        let res = self
            .command_channel
            .send(DroneCommand::SetPacketDropRate(pdr))
            .map_err(|err| err.to_string());
        if res.is_ok() {
            self.pdr = pdr;
        };
        res
    }
}
