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
    let history_state = use_state(|| vec![Frame::default()]);
    let step_number_state = use_state(|| 0);
    let x_is_next_state = use_state(|| true);

    let current = (*history_state)
        .last()
        .expect("This should not be empty ever.");
    let winner = calculate_winner(&current.squares);

    let moves: Vec<Html> = (*history_state)
        .clone()
        .iter()
        .enumerate()
        .map(|(i, _step)| {
            let desc = if i != 0 {
                format!("Go to move #{i}")
            } else {
                "Go to game start".to_string()
            };

            let step_number_state = step_number_state.clone();
            let x_is_next_state = x_is_next_state.clone();

            let jump_to = move |step: usize| {
                step_number_state.set(step);
                x_is_next_state.set((step % 2) == 0);
            };

            let onclick = Callback::from(move |_: MouseEvent| jump_to(i));

            html! {
                <li>
                    <button {onclick}>{desc}</button>
                </li>
            }
        })
        .collect();

    let handle_click = {
        let history_state = history_state.clone();
        let step_number_state = step_number_state.clone();
        let x_is_next_state = x_is_next_state.clone();

        Callback::from(move |i: usize| {
            let history_state = history_state.clone();
            let step_number_state = step_number_state.clone();
            let x_is_next_state = x_is_next_state.clone();

            Callback::from(move |_: MouseEvent| {
                let mut history = history_state
                    .get(0..((*step_number_state) + 1))
                    .expect("This should always work")
                    .to_vec();
                let current = history.last().expect("This should never be empty").clone();
                let mut squares = current.squares.clone();

                if calculate_winner(&squares).is_some() || squares[i].is_some() {
                    return;
                }

                squares[i] = if *x_is_next_state {
                    Some(AttrValue::Static("X"))
                } else {
                    Some(AttrValue::Static("O"))
                };

                history.push(Frame { squares });
                {
                    let history = history.clone();
                    history_state.set(history);
                }

                step_number_state.set(history.len() - 1);

                x_is_next_state.set(!(*x_is_next_state));
            })
        })
    };

    let status = match winner {
        Some(winner) => format!("Winner: {winner}"),
        None => format!("Next player: {}", if *x_is_next_state { "X" } else { "O" }),
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
