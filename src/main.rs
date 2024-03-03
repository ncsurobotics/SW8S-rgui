#![allow(non_snake_case)]

use const_format::concatcp;
use dioxus::prelude::*;
use dioxus_desktop::{window, WindowEvent};
use log::{info, LevelFilter};

const VIDEO_BASE: &str = "http://localhost:8888/";
const VIDEO_FRONT: &str = concatcp!(VIDEO_BASE, "mystream");
const VIDEO_BOTTOM: &str = concatcp!(VIDEO_BASE, "mystream");
const VIDEO_WIDTH_BASE: i64 = 800;
const VIDEO_HEIGHT_BASE: i64 = 600;
const VIDEO_PADDING: i64 = 10;
const VIDEO_TOTAL_WIDTH_BASE: i64 = (VIDEO_WIDTH_BASE + VIDEO_PADDING) * 2;
const SMALL_BUTTONS_MARGIN: i64 = 10;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    LaunchBuilder::desktop().launch(app);
}

fn app() -> Element {
    info!("Rebuilding DOM");
    let mut window_size = use_signal(|| window().inner_size());
    let window_video_ratio =
        f32::ceil((VIDEO_TOTAL_WIDTH_BASE as f32) / (window_size.read().width as f32)) as i64;
    let video_width = VIDEO_WIDTH_BASE / window_video_ratio;
    let video_height = VIDEO_HEIGHT_BASE / window_video_ratio;
    let total_width = (video_width * 2) + VIDEO_PADDING;

    // Redraw element sizes when window is resized
    window().create_wry_event_handler(move |event, _| {
        if let tao::event::Event::WindowEvent {
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

    rsx! {
        div {
            height : "100vh",
            width : "100vw",
            left : "0",
            top : "0",
            background_color : "gray",
            position : "absolute",
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
                        },
                        b {
                            "Return/Exit"
                        }
                    }
                }
            }
        }
    }
}
