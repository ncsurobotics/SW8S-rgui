use dioxus::prelude::*;
use dioxus_desktop::window;
use log::info;

use crate::util::{wrap_background, MainWinState, TestProps};

/// Run dry test suite
pub fn dry_test(cx: TestProps) -> Element {
    info!("Rendering test view");

    // Rebind from context
    let main_win_state = cx.main_win_state;

    // Close window if we're no longer in testing
    if *main_win_state.read() != MainWinState::Test {
        window().close();
    }

    rsx! { { wrap_background( dry_test_inner(main_win_state) ) } }
}

pub fn dry_test_inner(_main_win_state: Signal<MainWinState>) -> Element {
    rsx! {
        "TODO"
    }
}
