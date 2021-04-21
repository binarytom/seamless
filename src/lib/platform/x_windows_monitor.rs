// #![feature(type_alias_impl_trait)]
// type X_Connection = impl Connection + Send + Sync;

use crate::lib::platform::monitor::WindowMonitor;
use crate::lib::platform::window::Window;
use async_std::{io, task};
use std::rc::Rc;
use std::sync::mpsc;

use x11rb::connection::Connection;
use x11rb::errors::{ConnectError, ReplyOrIdError};
use x11rb::protocol::xfixes::{hide_cursor, query_version};
use x11rb::protocol::xinput::{
    list_input_devices, xi_select_events, EventMask, XIEventMask, BUTTON_PRESS_EVENT,
    BUTTON_RELEASE_EVENT, DEVICE_MOTION_NOTIFY_EVENT,
};
use x11rb::protocol::xproto;
use x11rb::protocol::xproto::{grab_pointer, warp_pointer};
use x11rb::protocol::Event;
use x11rb::COPY_DEPTH_FROM_PARENT;
type XConnection = impl Connection + Send + Sync;

pub struct XWindows {
    conn: XConnection,
    screen_num: usize,
}

impl XWindows {
    pub fn test() -> (XConnection, usize) {
        let (conn, screen_num) = x11rb::connect(None).unwrap();
        (conn, screen_num)
    }
    pub fn new() -> XWindows {
        // let (conn, screen_num): (XConnection, usize) = x11rb::connect(None).unwrap();
        let (conn, screen_num) = XWindows::test();
        XWindows { conn, screen_num }
    }

    /// 隐藏cursor
    pub fn x_hide_cursor(&self) {
        // let (conn, screen_num) = x11rb::connect(None).unwrap();
        let screen = &self.conn.setup().roots[self.screen_num];

        query_version(&self.conn, 4, 0);
        hide_cursor(&self.conn, screen.root);
        self.conn.flush();
        loop {}
    }

    ///// grap_pointer
    // fn x_grap_pointer(conn: &Connection, screen_num: usize) {
    //     // let (conn, screen_num) = x11rb::connect(None).unwrap();
    //     let screen = conn.setup().roots[screen_num];

    //     let GRAB_MASK: u16 =
    //         u32::from(xproto::EventMask::BUTTON_PRESS | xproto::EventMask::POINTER_MOTION) as u16;
    //     grab_pointer(
    //         conn,
    //         false,
    //         screen.root,
    //         GRAB_MASK,
    //         xproto::GrabMode::ASYNC,
    //         xproto::GrabMode::ASYNC,
    //         screen.root,
    //         x11rb::NONE,
    //         x11rb::CURRENT_TIME,
    //     );
    //     conn.flush();
    // }

    // /// warp_pointer, 修改指针位置
    // fn x_warp_pointer(conn: &Connection, screen_num: usize, x: i16, y: i16) {
    //     // let (conn, screen_num) = x11rb::connect(None).unwrap();
    //     let screen = conn.setup().roots[screen_num];

    //     warp_pointer(conn, x11rb::NONE, screen.root, 0, 0, 0, 0, x, y);
    //     conn.flush();
    // }

    // /// 窗口事件循环
    // fn x_window_event_loop() {
    //     loop {
    //         let event = conn.wait_for_event().unwrap();
    //         match event {
    //             Event::MotionNotify(e) => {
    //                 println!("input motion x={:?} y={:?}", e.root_x, e.root_y);
    //             }
    //             Event::XinputButtonPress(e) => println!("button press"),
    //             Event::XinputButtonRelease(e) => println!("button release"),
    //             _ => println!(""),
    //         }
    //     }
    // }
}

impl WindowMonitor for XWindows {
    fn start(&self) -> mpsc::Receiver<&str> {
        let (tx, rx) = mpsc::channel();

        rx
    }

    fn finished(&self) {}
}

impl Window for XWindows {
    fn hide_cursor(&self) {
        self.x_hide_cursor();
    }
    fn grap_pointer(&self) {}
    fn window_event_loop(&self) {}
    fn warp_pointer(&self, x: i16, y: i16) {}
}
