use crate::bus::BusStepResult;
use std::io;

pub struct InstrumentationTraceMacrocell {
    file: Option<Box<io::Write + 'static>>,
}

fn make_header(port: u8, payload_size: usize) -> u8 {
    let ss = match payload_size {
        1 => 1,
        2 => 2,
        4 => 3,
        _ => unreachable!(),
    };

    ((port & 0b11111) << 3) + ss
}

fn make_instrumentation_packet(port: u8, payload: &[u8]) -> Vec<u8> {
    // header is single byte
    // ----------------------
    // bits 0..=1 : SS, payload size = (1 << (SS -1)) bytes
    // bit 2 = 0
    // bits 3..=7: A[4:0], port number, 0-31
    //
    // followed by payload size bytes of payload, lsb order
    let mut packet = Vec::new();
    packet.push(make_header(port, payload.len()));
    packet.extend(payload);
    packet
}

impl InstrumentationTraceMacrocell {
    pub fn new(itm_file: Option<Box<io::Write + 'static>>) -> InstrumentationTraceMacrocell {
        InstrumentationTraceMacrocell { file: itm_file }
    }

    fn write_packet(&mut self, packet: Vec<u8>) {
        if let Some(f) = &mut self.file {
            f.write_all(packet.as_slice()).unwrap();
            f.flush().unwrap();
        }
    }

    pub fn read_stim0(&self) -> u32 {
        // return 0 if fifo is full, 1 otherwise
        1
    }

    pub fn write_stim_u32(&mut self, port: u8, value: u32) {
        let payload: [u8; 4] = [
            (value & 0xff) as u8,
            ((value & 0xff00) >> 8) as u8,
            ((value & 0xff_0000) >> 16) as u8,
            ((value & 0xff00_0000) >> 24) as u8,
        ];
        self.write_packet(make_instrumentation_packet(port, &payload));
    }

    pub fn write_stim_u16(&mut self, port: u8, value: u16) {
        let payload: [u8; 2] = [(value & 0xff) as u8, ((value & 0xff00) >> 8) as u8];
        self.write_packet(make_instrumentation_packet(port, &payload));
    }

    pub fn write_stim_u8(&mut self, port: u8, value: u8) {
        let payload: [u8; 1] = [value];
        self.write_packet(make_instrumentation_packet(port, &payload));
    }

    pub fn step(&mut self) -> BusStepResult {
        BusStepResult::Nothing
    }
}
