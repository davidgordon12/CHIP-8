#![allow(dead_code)] // allowed for now
#![allow(unused_variables)] // allowed for now 
const SCREEN_WIDTH: usize     =  64;
const SCREEN_HEIGHT: usize    =  32;
const NUM_V_REG: usize        =  16;
const NUM_KEYS: usize         =  16;
const RAM_SIZE: usize         =  4096;
const STACK_SIZE: usize       =  16;
const FONTSET_SIZE: u16       =  80;
const MAX_PROGRAM_MEMORY: u16 =  3584;
const PROGRAM_START_ADDR: u16 =  0x200;
const PROGRAM_END_ADDR: u16   =  0xFFF;

pub struct Emulator {
    pc: u16,
    sp: u16,
    ram: [u8; RAM_SIZE],
    v: [u8; NUM_V_REG],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    index: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

impl Emulator {
    fn new() -> Self {
        Self {
            pc: PROGRAM_START_ADDR,
            ram: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            v: [0; NUM_V_REG],
            keys: [false; NUM_KEYS],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            index: 0,
            sp: 0,
            dt: 0,
            st: 0,
        }
    }

    fn push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}

fn main() {
    let mut emulator = Emulator::new();

    emulator.push(30);

    let x = emulator.pop();

    println!("{}", x)
}
