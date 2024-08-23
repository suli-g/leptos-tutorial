use leptos::*;
use rand::Rng;

/*
Adds a progress bar with a maximum count of 100.
*/
#[component]
fn ProgressBar(
    #[prop(default = 100)] // Replace default with 'optional' if no default needed.
    max: u16,
    progress: ReadSignal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
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
    let mut rng = rand::thread_rng();
    let max_value = 20;
    let (count, set_count) = create_signal(0);
    let guess: i32 = rng.gen_range(0..max_value);
    view! {
        <ProgressBar
            progress=count
        />
        <div
            class="counter"
            class=("blue-21", move || count() == guess)  // Changes the class based on the value of count.
            // Change to style based on the value of count:
            style=move || format!("border-radius: calc(var(--base-radius) + {0}px) ; background-color: rgba(0, calc({0} * 25), 0, calc({0} / 100))", <i32>::abs(count() - guess))
            style:border=move || if count() == guess { "1px solid red" } else {"none"}
        >
        <h1
        
        >{count}</h1> // Identical to {move || count()}
        </div>
        <fieldset class="buttons">
        <button
        on:click=move |_| {
            set_count.update(|n| *n += 1)
        }
        >
        "+"
        </button>
        <button
        on:click=move |_| {
            set_count.update(|n| *n -= 1)
        }
        >
        "-"
        </button>
        </fieldset>
    }
}

fn main() {
    // Enable understandable errors in the browser console.
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
