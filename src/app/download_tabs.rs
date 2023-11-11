use dioxus::prelude::*;

use crate::app::download_tabs::{
    download_album::AlbumDownloadForm, download_playlist::PlaylistDownloadForm,
    download_radio::RadioDownloadForm,
};

mod download_album;
mod download_form;
mod download_playlist;
mod download_radio;

pub fn DownloadTabs(cx: Scope) -> Element {
    let index = use_state(cx, || 0);
    cx.render(rsx! {
        div { class: "flex flex-col",
            div { class: "flex",
                TabHeader { active: *index == 0, text: "Radio", on_click: move |_| index.set(0) }
                TabHeader { active: *index == 1, text: "Playlist", on_click: move |_| index.set(1) }
                TabHeader { active: *index == 2, text: "Album", on_click: move |_| index.set(2) }
            }
            div {
                TabContent { active: *index == 0, RadioDownloadForm {} }
                TabContent { active: *index == 1, PlaylistDownloadForm {} }
                TabContent { active: *index == 2, AlbumDownloadForm {} }
            }
        }
    })
}

#[component]
pub fn TabHeader<'a>(
    cx: Scope,
    active: bool,
    text: &'a str,
    on_click: EventHandler<'a, MouseEvent>,
) -> Element {
    cx.render(rsx! {button { onclick: move |evt| on_click.call(evt), class: format_args!("{} p-2 ", if *active { "bg-gray-100 boder-b" } else { "bg-white" }), "{text}" }})
}

#[component]
pub fn TabContent<'a>(cx: Scope, active: bool, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div { class: format_args!("{} p-2 ", if *active { "block" } else { "hidden" }), children }
    })
}
