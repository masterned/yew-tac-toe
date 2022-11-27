use yew::{function_component, html, use_state, AttrValue, Callback, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
struct SquareProps {
    value: Option<AttrValue>,
    onclick: Callback<MouseEvent, ()>,
}

#[function_component]
fn Square(SquareProps { value, onclick }: &SquareProps) -> Html {
    html! {
        <button class="square" {onclick}>
            { value.to_owned().unwrap_or_default() }
        </button>
    }
}

fn calculate_winner<'a>(squares: &'a [Option<AttrValue>; 9]) -> &'a Option<AttrValue> {
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
            return &squares[a];
        }
    }

    &None
}

#[derive(Properties, PartialEq)]
struct BoardProps {
    squares: [Option<AttrValue>; 9],
    handle_click: Callback<usize, Callback<MouseEvent>>,
}

#[function_component]
fn Board(
    BoardProps {
        squares,
        handle_click,
    }: &BoardProps,
) -> Html {
    let render_square = {
        let squares = squares.clone();
        move |i: usize| {
            let value = &squares[i];
            let onclick = handle_click.emit(i);
            html! { <Square {value} {onclick} /> }
        }
    };

    html! {
        <div>
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

#[derive(Clone)]
struct Frame {
    pub squares: [Option<AttrValue>; 9],
}

impl Default for Frame {
    fn default() -> Self {
        Frame {
            squares: [None, None, None, None, None, None, None, None, None],
        }
    }
}

#[function_component]
fn Game() -> Html {
    let history = use_state(|| vec![Frame::default()]);
    let x_is_next = use_state(|| true);

    let current = (*history).last().expect("This should not be empty ever.");
    let winner = calculate_winner(&current.squares);

    let moves: Vec<Html> = (*history)
        .clone()
        .iter()
        .enumerate()
        .map(|(i, _step)| {
            let desc = if i != 0 {
                format!("Go to move #{i}")
            } else {
                "Go to game start".to_string()
            };

            let onclick =
                Callback::from(move |_: MouseEvent| log::info!("#{} has been clicked", i));

            html! {
                <li>
                    <button {onclick}>{desc}</button>
                </li>
            }
        })
        .collect();

    let handle_click = {
        let history = history.clone();
        let x_is_next = x_is_next.clone();
        let current = current.clone();

        Callback::from(move |i: usize| {
            let history = history.clone();
            let x_is_next = x_is_next.clone();
            let current = current.clone();

            Callback::from(move |_: MouseEvent| {
                let mut squares = current.squares.clone();

                if calculate_winner(&squares).is_some() || squares[i].is_some() {
                    return;
                }

                squares[i] = if *x_is_next {
                    Some(AttrValue::Static("X"))
                } else {
                    Some(AttrValue::Static("O"))
                };

                let mut hist_clone = (*history).clone();
                hist_clone.push(Frame { squares });
                history.set(hist_clone);

                x_is_next.set(!(*x_is_next));
            })
        })
    };

    let status = match winner {
        Some(winner) => format!("Winner: {winner}"),
        None => format!("Next player: {}", if *x_is_next { "X" } else { "O" }),
    };

    let squares = current.squares.clone();
    html! {
        <div class="game">
            <div class="game-board">
                <Board {squares} {handle_click}/>
            </div>
            <div class="game-info">
                <div>{ status }</div>
                <ol>{ moves }</ol>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Game>::new().render();
}
