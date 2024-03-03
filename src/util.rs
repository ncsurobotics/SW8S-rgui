use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MainWinState {
    Planning,
    Live,
    Test,
}

#[derive(Copy, Clone, Props, PartialEq)]
pub struct TestProps {
    pub main_win_state: Signal<MainWinState>,
}

// Apply full screen gray background
pub fn wrap_background(inner: Element) -> Element {
    rsx! {
        div {
            height : "100vh",
            width : "100vw",
            left : "0",
            top : "0",
            background_color : "gray",
            position : "absolute",
            { inner }
        }
    }
}
