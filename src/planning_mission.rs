use dioxus::prelude::*;
use log::info;

use crate::util::MainWinState;

pub fn planning_mission(total_width: i64, mut main_win_state: Signal<MainWinState>) -> Element {
    rsx! {
        div {
            display : "flex",
            justify_content: "space-between",
            width : total_width.to_string() + "px",
            button {
                style : "font-size : 30px",
                width : "250px",
                onclick : move |_| {
                    info!("Selected Test");
                    main_win_state.set(MainWinState::Test);
                },
                b {
                    "Dry Test"
                }
            }
            button {
                style : "font-size : 30px",
                width : "250px",
                onclick : move |_| {
                    info!("Selected Deploy");
                    main_win_state.set(MainWinState::Live);
                },
                b {
                    "Deploy Mission"
                }
            }
        }
    }
}
