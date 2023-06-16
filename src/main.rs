use native_dialog::FileDialog;

mod emu;


fn main() {
    let result = FileDialog::new()
        .set_location("~")
        .add_filter("CHIP-8 ROMS", &["ch8"])
        .show_open_single_file()
        .unwrap()
        .unwrap();

    let rom_path: String = String::from(result.to_str().unwrap());

    let mut emulator: emu::Machine = emu::Machine::new();

    emulator
        .init(rom_path)
        .run();
}
