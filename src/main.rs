extern crate piston_window;

use pistoin_window::*;

struct App {}
impl App {
    fn render(&mut self, args: &RenderArgs) {}
    fn update(&mut self, args: &UpdateArgs) {}
}
fn main() {
    let mut window = WindowSettings::new("Reversi", [640, 480])
        .exit_on_exc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        if let Some(render) = e.render_args() {
            app.render(&render);
        }
        if let Some(update) = e.update_args() {
            app.update(&update);
        }
    }
}
