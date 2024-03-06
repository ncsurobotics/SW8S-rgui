#![allow(non_snake_case)]

mod config;
mod live_mission;
mod planning_mission;
mod test;
mod util;

use std::sync::Mutex;

use dioxus::prelude::*;
use dioxus_desktop::tao::event::Event;
use dioxus_desktop::{window, Config, WindowEvent};
use log::{info, LevelFilter};

use lazy_static::lazy_static;
use live_mission::live_mission;
use planning_mission::planning_mission;
use test::dry_test;
use util::MainWinState;

use crate::util::{wrap_background, TestProps};

const VIDEO_WIDTH_BASE: i64 = 800;
const VIDEO_HEIGHT_BASE: i64 = 600;
const VIDEO_PADDING: i64 = 10;
const VIDEO_TOTAL_WIDTH_BASE: i64 = (VIDEO_WIDTH_BASE + VIDEO_PADDING) * 2;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    LaunchBuilder::desktop().launch(app);
}

lazy_static! {
    static ref TEST_WINDOW_EXISTS: Mutex<bool> = Mutex::new(false);
}

fn app() -> Element {
    info!("Rebuilding DOM");

    let mut window_size = use_signal(|| window().inner_size());

    // Shrink components (on integer ratio) according to width undersize
    let window_video_ratio =
        f32::ceil((VIDEO_TOTAL_WIDTH_BASE as f32) / (window_size.read().width as f32)) as i64;
    let video_width = VIDEO_WIDTH_BASE / window_video_ratio;
    let video_height = VIDEO_HEIGHT_BASE / window_video_ratio;
    let total_width = (video_width * 2) + VIDEO_PADDING;

    // Redraw element sizes when window is resized
    #[allow(clippy::collapsible_match)] // WONTFIX
    window().create_wry_event_handler(move |event, _| {
        if let Event::WindowEvent {
            event: win_event, ..
        } = event
        {
            if let WindowEvent::Resized(resize_event) = win_event {
                // Prevent dupe triggers
                if *window_size.read() != *resize_event {
                    info!("Resize event!");
                    window_size.set(*resize_event);
                }
            }
        };
    });

    let main_win_state = use_signal(|| MainWinState::Planning);

    use_memo(move || {
        let mut test_window_exists = TEST_WINDOW_EXISTS.lock().unwrap();
        if *main_win_state.read() == MainWinState::Test {
            if !(*test_window_exists) {
                *test_window_exists = true;
                window().new_window(
                    VirtualDom::new_with_props(dry_test, TestProps { main_win_state }),
                    Config::default(),
                );
            }
        } else {
            *test_window_exists = false;
        }
    });

    rsx! {
        {
            wrap_background(
                match *main_win_state.read() {
                    MainWinState::Planning => planning_mission(total_width, main_win_state),
                    MainWinState::Live | MainWinState::Test => live_mission(video_width, video_height, total_width, main_win_state),
                }
            )
        }
    }
}
