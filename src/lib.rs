use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::MouseEvent;

use percy_dom::prelude::*;

#[wasm_bindgen]
struct App {
  dom_updater: DomUpdater,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> App {
        let start_view = html! { <div class=["start"]></div> };

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let mut dom_updater = DomUpdater::new_append_to_mount(start_view, &body);

        let info = vec![
            "Welcome!",
            "Rust WASM Demo"
        ];

        let button_text = "Check the console";

        let component = html! { <div id=["component"]>Component</div> };

        let end_view = html! {
            <div class=["container"]>
              <h1>{ info[0] }</h1>
              <h2>{ info[1] }</h2>

              <button onclick=|_event: MouseEvent| {
                   web_sys::console::log_1(&"Console logging".into());
                }
              >
                { button_text }
              </button>
              
              <div class="list">
                <ul>
                  <li>{ component }</li>
                </ul>
              </div>
            </div>
        };

        dom_updater.update(end_view);

        App { dom_updater }
    }
}