use leptos::prelude::*;
use web_sys::window;

#[component]
pub fn ConfigOutput(
    config_text: Memo<String>,
    all_filled: Memo<bool>,
) -> impl IntoView {
    let (copied, set_copied) = signal(false);

    let copy_click = move |_| {
        let text = config_text.get();
        if text.is_empty() {
            return;
        }
        if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) {
            let _ = clipboard.write_text(&text);
        }
        set_copied.set(true);
        set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
    };

    let btn_text = move || {
        if copied.get() {
            "Copied!".to_string()
        } else {
            "Copy".to_string()
        }
    };

    let btn_class = move || {
        let mut c = "copy-btn".to_string();
        if copied.get() {
            c.push_str(" copied");
        }
        c
    };

    view! {
        <div class="config-section">
            <div class="config-header">
                <h2>"Config"</h2>
                <span class=move || {
                    if all_filled.get() { "status-good" } else { "status-bad" }
                }>
                    {move || {
                        if all_filled.get() {
                            "✓ Ready to copy"
                        } else {
                            "✗ Fill in all Flow and PA values"
                        }
                    }}
                </span>
                <button
                    class=btn_class
                    disabled=move || !all_filled.get()
                    on:click=copy_click
                >
                    {btn_text}
                </button>
            </div>
            <textarea
                readonly
                spellcheck=false
                prop:value=move || config_text.get()
            ></textarea>
        </div>
    }
}
