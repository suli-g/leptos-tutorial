use leptos::*;

#[component]  // Function arguments are available as a props struct.
fn App() -> impl IntoView {
    let (count , set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1)
            }
            class:red=move || count() % 2 == 1  // Changes the class based on the value of count.
        >
            "Click me: "
            {count} // Identical to {move || count()}
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}