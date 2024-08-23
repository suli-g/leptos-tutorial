use leptos::*;

/*
Adds a progress bar with a maximum count of 100.
*/
#[component]
fn ProgressBar(progress: ReadSignal<i32>) -> impl IntoView {
    view! {
        <progress
            max="100"
            value=progress
        />
    }
}

/*
The main component, demonstrating how attributes can dynamically
be manipulated:
 */
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <ProgressBar
            progress=count
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
