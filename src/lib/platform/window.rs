pub trait Window {
    fn hide_cursor(&self);
    fn grap_pointer(&self);
    fn window_event_loop(&self);
    fn warp_pointer(&self, x: i16, y: i16);
}
