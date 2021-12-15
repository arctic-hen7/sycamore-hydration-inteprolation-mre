use sycamore::prelude::*;

/// A simple component that interpolates a set value.
#[component(HelloWorld<G>)]
fn hello_world() -> View<G> {
    let value = "Hello World!";

    view! {
        // The interpolated value will appear twice within this `p` element
        p { (value) }
    }
}

fn main() {
    // Prerender our component
    let prerendered = sycamore::render_to_string(|| view! { HelloWorld() });
    // Insert that directly into the HTML of our page
    let body = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("body")
        .unwrap()
        .unwrap();
    body.set_inner_html(&prerendered);

    // Now hydrate that
    sycamore::hydrate(|| view! { HelloWorld() });
}
