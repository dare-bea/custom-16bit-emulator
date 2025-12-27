use std::{fmt, io::Result};

#[derive(Debug)]
pub struct PortHandler {
    interrupts: Vec<bool>,
    ports: Vec<Box<dyn Port>>,
}

impl PortHandler {
    pub fn new() -> Self {
        Self {
            interrupts: vec![],
            ports: vec![]
        }
    }

    pub fn set_irq(&mut self, port: u8) {
        self.interrupts[port as usize] = true;
    }

    pub fn clear_irq(&mut self, port: u8) {
        self.interrupts[port as usize] = false;
    }

    pub fn get_irq(&self, port: u8) -> bool {
        self.interrupts[port as usize]
    }

    pub fn check_interrupt(&self) -> Option<u8> {
        for (port, irq) in self.interrupts.iter().enumerate() {
            if *irq {return Some(port as u8);}
        }
        None
    }

    pub fn acknowledge(&mut self, port: u8) -> Result<()> {
        self.clear_irq(port);
        self.ports[port as usize].acknowledge()
    }
}

pub trait Port: fmt::Debug {
    fn next(&mut self) -> Result<u8>;
    fn acknowledge(&mut self) -> Result<()>;
}