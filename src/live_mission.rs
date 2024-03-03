use const_format::concatcp;
use dioxus::prelude::*;
use log::info;

use crate::{util::MainWinState, VIDEO_PADDING};

const VIDEO_BASE: &str = "http://localhost:8888/";
const VIDEO_FRONT: &str = concatcp!(VIDEO_BASE, "mystream");
const VIDEO_BOTTOM: &str = concatcp!(VIDEO_BASE, "mystream");
const SMALL_BUTTONS_MARGIN: i64 = 10;

pub fn live_mission(
    video_width: i64,
    video_height: i64,
    total_width: i64,
    mut main_win_state: Signal<MainWinState>,
) -> Element {
    info!("Rendering live mission");

    rsx! {
        div {
            display : "flex",
            justify_content : "center",
            align_items : "center",
            flex_direction : "column",
            height : "98%",
            div {
                display : "flex",
                justify_content : "center",
                div {
                    display : "flex,table-row",
                    "justify-content": "center",
                    margin : "auto",
                    border_spacing : VIDEO_PADDING.to_string() + "px",
                    div {
                        display : "table-cell",
                        "justify-content": "center",
                        margin : "auto",
                        h2 { style: "text-align: center", "Front" }
                        iframe {
                            width : video_width, height : video_height, scrolling : false, src : VIDEO_FRONT, allow : "*"
                        }
                    }
                    div {
                        display : "table-cell",
                        margin : "auto",
                        h2 { style: "text-align: center", "Bottom" }
                        iframe { width : video_width, height : video_height, scrolling : false, src : VIDEO_BOTTOM, allow : "*" }
                    }
                }
            }
            div {
                display : "flex",
                text_align: "left",
                border : "2px solid DarkGreen",
                width : total_width.to_string() + "px",
                height : "100%",
                position : "relative",
                background_color : "LightGray",
                padding_left : "5px",
                overflow_y : "auto",
                flex_direction : "column-reverse",
                p {
                    margin : "0px",

                    "Similique ut doloremque facilis in laboriosam sed voluptatum. Aut asperiores laborum voluptatum cupiditate fugiat. Veritatis autem saepe doloremque eveniet eos. Minus eum quos libero cupiditate."
                    br {}
                    "Similique ut doloremque facilis in laboriosam sed voluptatum. Aut asperiores laborum voluptatum cupiditate fugiat. Veritatis autem saepe doloremque eveniet eos. Minus eum quos libero cupiditate."
                    br {}
                    "Similique ut doloremque facilis in laboriosam sed voluptatum. Aut asperiores laborum voluptatum cupiditate fugiat. Veritatis autem saepe doloremque eveniet eos. Minus eum quos libero cupiditate."
                    br {}
                    "Similique ut doloremque facilis in laboriosam sed voluptatum. Aut asperiores laborum voluptatum cupiditate fugiat. Veritatis autem saepe doloremque eveniet eos. Minus eum quos libero cupiditate."
                    br {}
                    "Similique ut doloremque facilis in laboriosam sed voluptatum. Aut asperiores laborum voluptatum cupiditate fugiat. Veritatis autem saepe doloremque eveniet eos. Minus eum quos libero cupiditate."
                    br {}
                    "Similique ut doloremque facilis in laboriosam sed voluptatum. Aut asperiores laborum voluptatum cupiditate fugiat. Veritatis autem saepe doloremque eveniet eos. Minus eum quos libero cupiditate."
                    br {}
                    "Similique ut doloremque facilis in laboriosam sed voluptatum. Aut asperiores laborum voluptatum cupiditate fugiat. Veritatis autem saepe doloremque eveniet eos. Minus eum quos libero cupiditate."
                    br {}
                    "Similique ut doloremque facilis in laboriosam sed voluptatum. Aut asperiores laborum voluptatum cupiditate fugiat. Veritatis autem saepe doloremque eveniet eos. Minus eum quos libero cupiditate."
                    br {}
                    "Similique ut doloremque facilis in laboriosam sed voluptatum. Aut asperiores laborum voluptatum cupiditate fugiat. Veritatis autem saepe doloremque eveniet eos. Minus eum quos libero cupiditate."
                    br {}
                    "Similique ut doloremque facilis in laboriosam sed voluptatum. Aut asperiores laborum voluptatum cupiditate fugiat. Veritatis autem saepe doloremque eveniet eos. Minus eum quos libero cupiditate."
                    br {}
                    "Similique ut doloremque facilis in laboriosam sed voluptatum. Aut asperiores laborum voluptatum cupiditate fugiat. Veritatis autem saepe doloremque eveniet eos. Minus eum quos libero cupiditate."
                    br {}
                    "Similique ut doloremque facilis in laboriosam sed voluptatum. Aut asperiores laborum voluptatum cupiditate fugiat. Veritatis autem saepe doloremque eveniet eos. Minus eum quos libero cupiditate."
                    br {}
                    br {}

                    span {
                        style : "color: red",
                        b {
                        "> COMMAND OUT TO ROBOT"
                        }
                    }
                    br {}
                    "CLI output from robot"
                    br {}
                    "CLI output from robot #2"
                }
            }
            br {}
            div {
                display : "flex",
                justify_content: "space-between",
                width : total_width.to_string() + "px",
                button {
                    style : "font-size : 16px",
                    width : "150px",
                    margin_left : SMALL_BUTTONS_MARGIN.to_string() + "px",
                    margin_right : SMALL_BUTTONS_MARGIN.to_string() + "px",
                    onclick : move |_| {
                        info!("Selected Run Tests");
                        main_win_state.set(MainWinState::Test);
                    },
                    b {
                        "Run Tests"
                    }
                }
                button {
                    style : "font-size : 16px",
                    width : "150px",
                    margin_left : SMALL_BUTTONS_MARGIN.to_string() + "px",
                    margin_right : SMALL_BUTTONS_MARGIN.to_string() + "px",
                    onclick : move |_| {
                        info!("Selected Mission Graph");
                    },
                    b {
                        "Mission Graph"
                    }
                }
                button {
                    style : "font-size : 16px",
                    width : "150px",
                    margin_left : SMALL_BUTTONS_MARGIN.to_string() + "px",
                    margin_right : SMALL_BUTTONS_MARGIN.to_string() + "px",
                    onclick : move |_| {
                        info!("Selected Set Data Dir");
                    },
                    b {
                        "Set Data Dir"
                    }
                }
                button {
                    style : "font-size : 16px",
                    width : "150px",
                    margin_left : SMALL_BUTTONS_MARGIN.to_string() + "px",
                    margin_right : SMALL_BUTTONS_MARGIN.to_string() + "px",
                    onclick : move |_| {
                        info!("Selected PLACEHOLDER");
                    },
                    b {
                        "PLACEHOLDER"
                    }
                }
                button {
                    style : "font-size : 16px",
                    width : "150px",
                    margin_left : SMALL_BUTTONS_MARGIN.to_string() + "px",
                    margin_right : SMALL_BUTTONS_MARGIN.to_string() + "px",
                    onclick : move |_| {
                        info!("Selected PLACEHOLDER");
                    },
                    b {
                        "PLACEHOLDER"
                    }
                }
            }
            br {}
            div {
                display : "flex",
                justify_content: "space-between",
                width : total_width.to_string() + "px",
                button {
                    style : "font-size : 30px",
                    width : "250px",
                    onclick : move |_| {
                        info!("Selected Software ARM");
                    },
                    b {
                        "Software ARM"
                    }
                }
                button {
                    style : "font-size : 30px",
                    width : "250px",
                    onclick : move |_| {
                        info!("Selected Start Mission");
                    },
                    b {
                        "Start Mission"
                    }
                }
                button {
                    style : "font-size : 30px",
                    width : "250px",
                    onclick : move |_| {
                        info!("Selected Return/Exit");
                        cleanup();
                        main_win_state.set(MainWinState::Planning);
                    },
                    b {
                        "Return/Exit"
                    }
                }
            }
        }
    }
}

/// Turn robot fully off
fn cleanup() {}
