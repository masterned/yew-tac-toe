use yew::{function_component, html, Html};

#[function_component]
fn App() -> Html {
    html! { "Hello World" }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
