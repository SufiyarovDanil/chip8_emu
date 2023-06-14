mod emu;


fn main() {
    let mut emulator: emu::Machine = emu::Machine::new();
    let rom_name: String = String::from("D:/Projects/emus/chip-8-emu/res/IBM Logo.ch8");

    emulator
        .init(&rom_name)
        .unwrap();

    while emulator.is_running() {
        emulator.tick();
    }
}
