use chip8_emulator_rust::machine::Machine;
fn main() {
    let mut chip8 = Machine::new();
    chip8.load(String::from("./roms/Space Invaders [David Winter].ch8"));

    chip8.run();
}
