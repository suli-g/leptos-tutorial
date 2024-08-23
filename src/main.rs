use leptos::*;


/*
The main component, demonstrating how attributes can dynamically
be manipulated:
 */
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <progress
            max="100"
            value=double_count // derived signal.
        />
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1)
            }
            class:red=move || count() % 2 == 1
            class=("blue-21", move || count() % 3 == 1)  // Changes the class based on the value of count.
            // Change to style based on the value of count:
            style:background-color=move || format!("rgba({}, 0, 100, 0.{})", count(), count()) // dynamic attribute
        >
            "Click me: "
            {count} // Identical to {move || count()}
        </button>
    }
}

fn main() {
    // Enable understandable errors in the browser console.
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
