# Yew-Tac-Toe

Yew is similar to React in syntax and style, but Yew does not have an
in-depth tutorial like React. This project is the React tic-tac-toe
tutorial manually transpiled into Yew.

The semantic gap between Rust and JavaScript is quite large, and the
React tutorial cuts some corners for the sake of simplicity. As such,
many things do not directly translate from one framework to the other.
As with translating between spoken languages, idiom approximates must
be used.

There will be some differences between the original and the translation.
For instance, the translation leverages more use of functional
components than the original, and, as such, also takes advantage of
hooks. The original handles all of the logic in a single file &ndash; by the
end of the translation, it will be separated into multiple files for
readability and conventionality.
