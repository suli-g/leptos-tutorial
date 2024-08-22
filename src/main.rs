use leptos::*;

#[component]  // Function arguments are available as a props struct.
fn App() -> impl IntoView {
    let (count , set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count(3);
            }
        >
            "Click me: "
            {move || count()}
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}