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

fn calculate_winner(squares: [Option<&str>; 9]) -> Option<&str> {
    const LINES: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for line in LINES {
        let [a, b, c] = line;
        if squares[a] == squares[b] && squares[a] == squares[c] {
            return squares[a];
        }
    }

    None
}

#[function_component]
fn Board() -> Html {
    let x_is_next = use_state(|| true);
    let squares: UseStateHandle<[Option<&str>; 9]> = use_state(|| [None; 9]);

    let winner = calculate_winner(*squares);
    let status = match winner {
        Some(mark) => format!("Winner: {mark}"),
        None => format!("Next player: {}", if *x_is_next { "X" } else { "O" }),
    };

    let handle_click = |i: usize| {
        let squares = squares.clone();
        let x_is_next = x_is_next.clone();

        Callback::from(move |_| {
            if winner.is_some() {
                return;
            }

            let mut squares_clone = (*squares).clone();
            squares_clone[i] = if *x_is_next { Some("X") } else { Some("O") };
            squares.set(squares_clone);
            x_is_next.set(!(*x_is_next));
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
