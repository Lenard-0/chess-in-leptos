use leptos::{ev::select, prelude::*};

fn main() {
    leptos::mount::mount_to_body(ChessBoard)
}

#[derive(Debug, Clone)]
struct SelectedTile {
    row: usize,
    col: usize,
}

impl SelectedTile {
    fn is_selected(&self, row: usize, col: usize) -> bool {
        self.row == row && self.col == col
    }
}

#[component]
pub fn ChessBoard() -> impl IntoView {
    let (selected_tile, set_selected_tile) = signal(Some(SelectedTile {
        row: 0,
        col: 0
    }));
    set_selected_tile.set(None);
    let board: Vec<Vec<Option<&'static str>>> = vec![
        // Row 0: Black major pieces
        vec![Some("♜"), Some("♞"), Some("♝"), Some("♛"), Some("♚"), Some("♝"), Some("♞"), Some("♜")],
        // Row 1: Black pawns
        vec![Some("♟"), Some("♟"), Some("♟"), Some("♟"), Some("♟"), Some("♟"), Some("♟"), Some("♟")],
        // Rows 2-5: Empty rows
        vec![None; 8],
        vec![None; 8],
        vec![None; 8],
        vec![None; 8],
        // Row 6: White pawns
        vec![Some("♙"), Some("♙"), Some("♙"), Some("♙"), Some("♙"), Some("♙"), Some("♙"), Some("♙")],
        // Row 7: White major pieces
        vec![Some("♖"), Some("♘"), Some("♗"), Some("♕"), Some("♔"), Some("♗"), Some("♘"), Some("♖")],
    ];

    view! {
        <div class="chess-container">
            <div class="chessboard">
                {board.into_iter().enumerate().map(|(row_idx, row)| {
                    view! {
                        <div class="row">
                            {row.into_iter().enumerate().map(|(col_idx, piece)| {
                                // Determine square color: alternate based on row + col indices.
                                let square_class = if (row_idx + col_idx) % 2 == 0 { "light" } else { "dark" };
                                view! {
                                    <div
                                        class={move || format!(
                                            "square {} {}",
                                            square_class,
                                            match selected_tile.get() {
                                                Some(selected_tile) if selected_tile.is_selected(row_idx, col_idx) => "selected",
                                                _ => ""
                                            }
                                        )}
                                        on:click={move |_| set_selected_tile.set(Some(SelectedTile { row: row_idx, col: col_idx }))}
                                    >
                                        {move || piece.unwrap_or("")}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
        <style>
        {r#"
            .chess-container {
                text-align: center;
                margin: 20px;
            }

            .chessboard {
                display: inline-block;
                border: 2px solid #333;
            }

            .row {
                display: flex;
            }

            .square {
                width: 100px;
                height: 100px;
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 70px;
            }

            .square:hover {
                cursor: pointer;
            }

            .light {
                background-color: rgba(240, 217, 181, 1);
            }

            .light:hover {
                background-color: rgba(240, 217, 181, 0.8);
                background-color: rgba(255, 255, 255, 0.5);
            }

            .dark {
                background-color: rgba(181, 136, 99, 1);
            }

            .dark:hover {
                background-color: rgba(255, 255, 255, 0.5);
            }

            .selected, .selected:hover {
                background-color: rgba(255, 255, 0, 0.5);
            }
        "#}
    </style>
    }
}
