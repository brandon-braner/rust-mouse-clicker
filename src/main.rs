use enigo::{Enigo, MouseButton, MouseControllable};
use std::thread;
use std::time::Duration;

fn click_and_hold() {
    let mut enigo = Enigo::new();
    enigo.mouse_down(MouseButton::Left);
    thread::sleep(Duration::from_secs(15));
    enigo.mouse_up(MouseButton::Left);
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    thread::sleep(Duration::from_secs(5));
    loop {
        click_and_hold();
    }
}
