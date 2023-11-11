use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub(super) fn DownloadForm<'a>(
    cx: Scope,
    title: &'a str,
    onclick: EventHandler<'a, String>,
) -> Element {
    let track_url = use_state(cx, || "".to_string());

    cx.render(rsx!(
        div {
            h2 { class: "text-2xl", "{title}" }
            div { class: "flex flex-row gap-2 mt-1",
                input {
                    class: "p-2 bg-gray-100 rounded-md border border-black flex-grow",
                    r#type: "text",
                    style: "width: 400px",
                    value: "{track_url}",
                    oninput: move |e| {
                        track_url.set(e.value.clone());
                    }
                }
                button {
                    class: "bg-gray-200 hover:bg-gray-300 rounded-md p-2",
                    onclick: move |_| {
                        onclick.call(track_url.get().to_string());
                    },
                    "Start"
                }
            }
        }
    ))
}
