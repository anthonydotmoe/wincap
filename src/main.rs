#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod window;
mod resource_consts;
mod screen_view;

use w::InitCommonControlsEx;
use winsafe::{prelude::*, self as w};
use window::Window;


fn main() {
    if let Err(e) = run_app() {
        w::task_dlg::error(
            &w::HWND::NULL, "Unhandled error", None, &e.to_string())
            .unwrap();
    }
}

fn run_app() -> w::AnyResult<i32> {
    
    let mut icc = w::INITCOMMONCONTROLSEX::default();
    icc.icc = w::co::ICC::STANDARD_CLASSES;
    
    InitCommonControlsEx(&icc).unwrap();
    
    Window::new()
        .run()
        .map_err(|err| err.into())
}