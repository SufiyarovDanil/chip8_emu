use piston_window as pw;
use pw::{
    PistonWindow,
    WindowSettings,
    Transformed
};


#[allow(dead_code)]
pub struct Window {
    window: PistonWindow,
    width: u32,
    height: u32,
    x_scale: f64,
    y_scale: f64,
}


#[allow(dead_code)]
impl Window {
    pub fn new(title: String, width: u32, height: u32) -> Self {
        let window: PistonWindow = WindowSettings::new(title, (width, height))
        .exit_on_esc(true)
        .graphics_api(pw::OpenGL::V4_5)
        .build()
        .unwrap();

        Self {
            window,
            width,
            height,
            x_scale: width as f64 / 64.0,
            y_scale: height as f64 / 32.0
        }
    }

    pub fn update_screen(&mut self, display: &[bool]) {
        if let Some(e) = self.window.next() {
            self.window.draw_2d(&e, |c, g, _| {
                pw::clear([0.0, 0.0, 0.0, 1.0], g);

                for i in 0..display.len() {
                    if display[i] {
                        let x: f64 = (i as u32 % 64) as f64 * self.x_scale;
                        let y: f64 = (i as u32 / 64) as f64 * self.y_scale;
                        let c = c.trans(x, y);
                        let rect = [self.x_scale, self.x_scale, self.y_scale, self.y_scale];
                        pw::rectangle([0.0, 1.0, 0.0, 1.0], rect, c.transform, g);
                    }
                }
            });
        }
    }
}