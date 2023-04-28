mod emulator;
use crate::emulator::Emulator;

fn main() {
    let mut emulator = Emulator::new();
    emulator.reset();
}
