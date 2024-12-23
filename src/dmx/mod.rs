use crate::Context;
use raylib::prelude::*;

pub struct ChannelData {
    pub channel: u8,
    pub value: u8,
}

pub trait Fixture: std::fmt::Debug {
    fn channels(&self) -> Vec<ChannelData>;
    fn name(&self) -> String;
    fn config(&mut self, ctx: &mut Context<'_>);
    fn render(&mut self, ctx: &mut Context<'_>);
    fn controls(&self) -> Vec<Control>;
    fn apply_control(&mut self, control: &Control, value: u8);
}

pub enum Control {
    Slider { name: String, min: u8, max: u8 },
}

#[derive(Debug)]
pub struct Universe {
    pub universe: u8,
    pub name: String,
    pub fixtures: Vec<Box<dyn Fixture>>,
}

#[derive(Debug)]
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
                for channel in fixture.channels() {
                    packet[channel.channel as usize] = channel.value;
                }
            }
            packets.push((universe.universe, packet));
        }
        packets
    }
}
