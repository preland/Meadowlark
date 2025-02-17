use vizia::prelude::*;

use crate::{
    state_system::{AppAction, BrowserPanelAction, StateSystem, WorkingState},
    ui::generic_views::{Icon, IconCode},
};

use super::browser_panel::BrowserPanelLens;

pub fn side_tab_bar(cx: &mut Context) {
    const ICON_FRAME_SIZE: f32 = 26.0;
    const ICON_SIZE: f32 = 22.0;

    VStack::new(cx, |cx| {
        Button::new(
            cx,
            |cx| {
                cx.emit(AppAction::BrowserPanel(BrowserPanelAction::SetPanelShown(
                    !StateSystem::working_state
                        .then(WorkingState::browser_panel_lens)
                        .then(BrowserPanelLens::panel_shown)
                        .get(cx),
                )))
            },
            |cx| Icon::new(cx, IconCode::Folder, ICON_FRAME_SIZE, ICON_SIZE),
        )
        .class("side_tab_btn")
        .toggle_class(
            "side_tab_toggled",
            StateSystem::working_state
                .then(WorkingState::browser_panel_lens)
                .then(BrowserPanelLens::panel_shown),
        );

        Button::new(cx, |_| {}, |cx| Icon::new(cx, IconCode::Piano, ICON_FRAME_SIZE, ICON_SIZE))
            .class("side_tab_btn");

        Button::new(
            cx,
            |_| {},
            |cx| Icon::new(cx, IconCode::Properties, ICON_FRAME_SIZE, ICON_SIZE),
        )
        .class("side_tab_btn");
    })
    .height(Stretch(1.0))
    .row_between(Pixels(6.0))
    .width(Pixels(32.0))
    .class("side_tab_bar");
}
