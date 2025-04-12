use chip8_emulator_rust::machine::Machine;
fn main() {
    let mut chip8 = Machine::new();
    chip8.load(String::from("./roms/Most Dangerous Game [Peter Maruhnic].ch8"));

    chip8.run();
}
