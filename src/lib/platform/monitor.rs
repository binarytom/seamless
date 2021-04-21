use std::sync::mpsc;

pub trait WindowMonitor {
    fn start(&self) -> mpsc::Receiver<&str>;
    fn finished(&self);
    // fn on_mouse_move();
    // fn on_mouse_button_press();
    // fn on_mouse_button_release();
    // fn on_keyboard_press();
    // fn on_keyboard_release();
}
