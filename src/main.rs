use winit::{
    window::WindowBuilder,
    event_loop::EventLoop,
    event::{Event, WindowEvent}
};

mod emu;


fn main() {
    let event_loop: EventLoop<()> = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    let mut emulator: emu::Machine = emu::Machine::new();
    let rom_name: String = String::from("D:/Projects/emus/chip-8-emu/res/BC_test.ch8");

    emulator
        .init(&rom_name)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested, .. } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            },
            _ => {}
        }

        emulator.exec_instruction();
        std::thread::sleep(std::time::Duration::from_millis(100));
    });
}
