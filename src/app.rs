use mastodon_model::Status;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::toot::TootList;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[function_component(App)]
pub fn app() -> Html {
    let toots = use_state(Vec::new);
    {
        let toots = toots.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_toots = from_value::<Vec<Status>>(
                        invoke("fetch_public_timeline", JsValue::null()).await,
                    )
                    .unwrap();
                    log(&format!("Fetched {} toots", fetched_toots.len()));
                    toots.set(fetched_toots);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <main class="container">
            <TootList toots={(*toots).clone()} />
        </main>
    }
}
