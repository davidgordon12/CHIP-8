#![allow(dead_code)] // disabled for now
const SCREEN_WIDTH: u16       =  64;
const SCREEN_HEIGHT: u16      =  32;
const NUM_V_REG: u16          =  16;
const RAM_SIZE: u16           =  4096;
const STACK_SIZE: u16         =  16;
const FONTSET_SIZE: u16       =  80;
const MAX_PROGRAM_MEMORY: u16 =  3584;
const PROGRAM_START_ADDR: u16 =  0x200;
const PROGRAM_END_ADDR: u16   =  0xFFF;

pub struct CPU {
    pc: u16,
    sp: u16,
    ram: [u8; RAM_SIZE as usize],
    v: [u8; NUM_V_REG as usize],
}

fn main() {
    for i in PROGRAM_START_ADDR..=PROGRAM_END_ADDR {

    }
}
