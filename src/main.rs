use yew::{
    function_component, html, use_state, Callback, Html, MouseEvent, Properties, UseStateHandle,
};

#[derive(Properties, PartialEq)]
struct SquareProps {
    value: Option<String>,
    onclick: Callback<MouseEvent>,
}

#[function_component]
fn Square(SquareProps { value, onclick }: &SquareProps) -> Html {
    html! {
        <button class="square" {onclick}>
            { value.clone().unwrap_or("".to_string()) }
        </button>
    }
}

#[function_component]
fn Board() -> Html {
    let status = "Next player: X";
    let squares: UseStateHandle<[Option<&str>; 9]> = use_state(|| [None; 9]);

    let handle_click = |i: usize| {
        let squares = squares.clone();

        Callback::from(move |_| {
            let mut squares_clone = (*squares).clone();
            squares_clone[i] = Some("X");
            squares.set(squares_clone);
        })
    };

    let render_square = |i: usize| {
        let value = (*squares)[i];
        html! { <Square {value} onclick={handle_click(i)} /> }
    };

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
