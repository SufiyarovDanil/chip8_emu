mod emu;


fn main() {
    // let mut window: PistonWindow = WindowSettings::new("CHIP-8 EMULATOR", (600, 300))
    //     .exit_on_esc(true)
    //     .graphics_api(pw::OpenGL::V4_5)
    //     .build()
    //     .unwrap();

    let mut emulator: emu::Machine = emu::Machine::new();
    let rom_name: String = String::from("D:/Projects/emus/chip-8-emu/res/IBM Logo.ch8");

    emulator
        .init(&rom_name)
        .unwrap();

    // while let Some(e) = window.next() {
    //     window.draw_2d(&e, |c, g, _| {
    //         pw::clear([0.0, 0.0, 0.0, 1.0], g);
    //         for i in 0..5 {
    //             let c = c.trans(0.0, i as f64 * 100.0);
    //             let black = [0.0, 0.0, 0.0, 1.0];
    //             let red = [1.0, 0.0, 0.0, 1.0];
    //             let rect = pw::math::margin_rectangle([20.0, 20.0, 60.0, 60.0], i as f64 * 5.0);
    //             pw::rectangle(red, rect, c.transform, g);
    //             //pw::Rectangle::new_border(black, 2.0).draw(rect, &c.draw_state, c.transform, g);
    //         }
    //     });
    //     //emulator.exec_instruction();
    //     //std::thread::sleep(std::time::Duration::from_millis(100));
    // }

    while emulator.is_running() {
        emulator.tick();
    }
}
