use piston_window as pw;
use pw::{
    PistonWindow,
    WindowSettings,
    Transformed,
    PressEvent,
    ReleaseEvent,
    Button
};
use super::display;


#[allow(dead_code)]
pub struct Window {
    window: PistonWindow,
    x_scale: f64,
    y_scale: f64,
    pressed_button: Button,
    released_button: Button
}


impl Window {
    pub fn new(title: String, width: u32, height: u32) -> Self {
        let window: PistonWindow = WindowSettings::new(title, (width, height))
        .exit_on_esc(true)
        .graphics_api(pw::OpenGL::V4_5)
        .build()
        .unwrap();

        Self {
            window,
            x_scale: width as f64 / 64.0,
            y_scale: height as f64 / 32.0,
            pressed_button: Button::Keyboard(pw::Key::Unknown),
            released_button: Button::Keyboard(pw::Key::Unknown)
        }
    }

    pub fn update_screen(&mut self, display: &mut display::Display) {
        if let Some(e) = self.window.next() {
            if let Some(button) = e.press_args() {
                self.pressed_button = button;
            }

            if let Some(button) = e.release_args() {
                self.released_button = button;
            } 

            self.window.draw_2d(&e, |c, g, _| {
                pw::clear([0.0, 0.5, 0.0, 1.0], g);

                for i in 0..display.get_size() {
                    if display.read_pixel(i) {
                        let x: f64 = (i as u32 % 64) as f64 * self.x_scale;
                        let y: f64 = (i as u32 / 64) as f64 * self.y_scale;
                        let c = c.trans(x, y);
                        let rect = [self.x_scale, self.x_scale, self.y_scale, self.y_scale];
                        pw::rectangle([0.0, 0.0, 0.0, 1.0], rect, c.transform, g);
                    }
                }
            });
        }
    }

    pub fn get_presssed_key(&mut self) -> Button {
        let key = self.pressed_button;
        
        self.pressed_button = Button::Keyboard(pw::Key::Unknown);
        key
    }

    pub fn get_released_key(&mut self) -> Button {
        let key = self.released_button;

        self.released_button = Button::Keyboard(pw::Key::Unknown);
        key
    }
}
