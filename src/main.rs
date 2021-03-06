#![feature(str_split_whitespace_as_str)]
#![feature(iter_intersperse)]
#![feature(path_try_exists)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

use std::thread;

use druid::{AppLauncher, ExtEventSink, Rect, Target, WindowDesc, WindowId};

use crate::app_delegate::{Delegate, GLOBAL_HOT_KEY};
use crate::config::Settings;
use crate::data::AppState;

mod app_delegate;
mod command;
mod command_box;
mod config;
mod data;
mod document;
mod textarea_controller;
mod view;

pub fn main() {
    let screen_rect = if cfg!(windows) {
        druid::Screen::get_display_rect()
    } else {
        Rect::new(0_f64, 0_f64, 1600_f64, 1000_f64)
    };
    let win_height = screen_rect.height() * 0.777;

    let main_window = WindowDesc::new(view::ui_builder())
        .title("/dev/swap sandbox")
        // .transparent(true)
        // .show_titlebar(false)
        .set_position((100.0, screen_rect.height() - win_height - 10.0))
        .window_size((screen_rect.width() * 0.333, win_height));

    let mut data = AppState::default();
    data.replace_settings(Settings::load());

    println!("Settings: {:?}", data.get_settings());

    let window_id = main_window.id;
    let app = AppLauncher::with_window(main_window)
        .configure_env(|_e, _a| {})
        .log_to_console()
        .delegate(Delegate::new(window_id));

    let event_sink = app.get_external_handle();
    global_hotkey_listener(event_sink, window_id);

    app.launch(data).expect("launch failed");
}

fn global_hotkey_listener(sink: ExtEventSink, window_id: WindowId) {
    thread::spawn(move || {
        let mut hk = hotkey::Listener::new();
        hk.register_hotkey(
            hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
            'A' as u32,
            move || {
                println!("Ctrl-Shift-A pressed!");
                sink.submit_command(GLOBAL_HOT_KEY, window_id, Target::Auto)
                    .expect("command failed to submit");
            },
        )
        .expect("failed to register hotkey");

        hk.listen();
    });
}
