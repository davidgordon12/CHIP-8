#![allow(dead_code)]        // allowed for now
#![allow(unused_mut)]       // allowed for now
#![allow(unused_variables)] // allowed for now 
const SCREEN_WIDTH: usize     =  64;
const SCREEN_HEIGHT: usize    =  32;
const NUM_V_REG: usize        =  16;
const NUM_KEYS: usize         =  16;
const RAM_SIZE: usize         =  4096;
const STACK_SIZE: usize       =  16;
const FONTSET_SIZE: usize     =  80;
const MAX_PROGRAM_MEMORY: u16 =  3584;
const PROGRAM_START_ADDR: u16 =  0x200;
const PROGRAM_END_ADDR: u16   =  0xFFF;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0 
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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
       let mut emu = Self {
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
        };

        emu.ram[0..FONTSET_SIZE].copy_from_slice(&FONTSET);

        emu
    }

    fn reset(&mut self) {
        self.pc = PROGRAM_START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.stack = [0; STACK_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v = [0; NUM_V_REG];
        self.index = 0;
        self.sp = 0;
        self.dt = 0;
        self.st = 0;
        self.keys = [false; NUM_KEYS];
        self.ram[0..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    fn fetch(&mut self) -> u16 {
        let hi_byte = self.ram[self.pc as usize] as u16;
        let lo_byte = self.ram[(self.pc + 1) as usize] as u16;

        let op = (hi_byte << 8) | lo_byte;
        self.pc += 2;

        op
    }

    fn decode(&mut self, op: u16) -> &str {
        "CLS"
    }

    fn execute(&mut self, op: u16) {
       /* DECODE AND EXECUTE IN ONE FUNCTION TEMPORARILY */ 
    }

    fn cycle(&mut self) {
        // Fetch -> Decode -> Execute
        let op = self.fetch();

        
    }

    fn increment_timers(&mut self) {
        if self.dt > 0 { self.dt -= 1; }

        if self.st > 0 {
            if self.st == 1 {
                // beep
            }
            self.st -= 1; 
        }
    }

    fn push(&mut self, value: u16) {
        if self.sp >= 16 { panic("Cannot push onto stack, pointer at 16") }

        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        if self.sp <= 0 { panic("Cannot pop stack, pointer at 0.") }

        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}

fn panic(info: &str) -> ! {
    println!("Panic: {}", info);
    loop {}
}

fn main() {
    let mut emulator = Emulator::new();
    emulator.reset();
}
