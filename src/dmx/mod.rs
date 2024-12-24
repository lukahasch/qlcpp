use raylib::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelData {
    pub channel: u8,
    pub value: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fixture {
    pub channels: Vec<ChannelData>,
    pub name: String,
    pub controls: Vec<Control>,
}

impl Fixture {
    pub fn apply_control(&mut self, control: &Control, value: u8) {}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Control {
    Slider { name: String, min: u8, max: u8 },
    Color { name: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Universe {
    pub universe: u8,
    pub name: String,
    pub fixtures: Vec<Fixture>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DMX {
    pub universes: Vec<Universe>,
}

impl Default for DMX {
    fn default() -> Self {
        Self::new()
    }
}

impl DMX {
    pub fn new() -> Self {
        Self {
            universes: Vec::new(),
        }
    }

    pub fn build_packet(&self) -> Vec<(u8, [u8; 512])> {
        let mut packets = Vec::new();
        for universe in &self.universes {
            let mut packet = [0; 512];
            for fixture in &universe.fixtures {
                for channel in &fixture.channels {
                    packet[channel.channel as usize] = channel.value;
                }
            }
            packets.push((universe.universe, packet));
        }
        packets
    }
}
