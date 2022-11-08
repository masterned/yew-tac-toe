use yew::{function_component, html, use_state, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
struct SquareProps {
    value: i32,
}

#[function_component]
fn Square() -> Html {
    let value = use_state(|| None);

    let button_onclick = {
        let value = value.clone();

        Callback::from(move |_| {
            value.set(Some("X"));
        })
    };

    html! {
        <button class="square" onclick={button_onclick}>
            { *value }
        </button>
    }
}

fn render_square(_i: i32) -> Html {
    html! { <Square /> }
}

#[function_component]
fn Board() -> Html {
    let status = "Next player: X";

    html! {
        <div>
            <div class="status">{status}</div>
            <div class="board-row">
                { render_square(0) }
                { render_square(1) }
                { render_square(2) }
            </div>
            <div class="board-row">
                { render_square(3) }
                { render_square(4) }
                { render_square(5) }
            </div>
            <div class="board-row">
                { render_square(6) }
                { render_square(7) }
                { render_square(8) }
            </div>
        </div>
    }
}

#[function_component]
fn Game() -> Html {
    html! {
        <div class="game">
            <div class="game-board">
                <Board />
            </div>
            <div class="game-info">
                <div>{ "status" }</div>
                <ol>{ "todo" }</ol>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Game>::new().render();
}
