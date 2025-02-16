use leptos::{ev::select, prelude::*};
use rust_fish_chess_engine::{chess_functionality::moves::{calculate_possible_moves, king::CastleState, Move}, Piece, PieceType};
use web_sys::wasm_bindgen::JsValue;

fn main() {
    leptos::mount::mount_to_body(ChessBoard)
}

fn display_piece(piece: Piece) -> &'static str {
    match piece {
        Piece::White(piece_type) => match piece_type {
            PieceType::King => "♔",
            PieceType::Queen => "♕",
            PieceType::Rook => "♖",
            PieceType::Bishop => "♗",
            PieceType::Knight => "♘",
            PieceType::Pawn => "♙",
        },
        Piece::Black(piece_type) => match piece_type {
            PieceType::King => "♚",
            PieceType::Queen => "♛",
            PieceType::Rook => "♜",
            PieceType::Bishop => "♝",
            PieceType::Knight => "♞",
            PieceType::Pawn => "♟",
        },
    }
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

    let (white_turn, set_white_turn) = signal(true);

    set_selected_tile.set(None);
    let (board, set_board) = signal(vec![
            // Row 0: Black major pieces
            vec![Some(Piece::Black(PieceType::Rook)), Some(Piece::Black(PieceType::Knight)), Some(Piece::Black(PieceType::Bishop)), Some(Piece::Black(PieceType::Queen)), Some(Piece::Black(PieceType::King)), Some(Piece::Black(PieceType::Bishop)), Some(Piece::Black(PieceType::Knight)), Some(Piece::Black(PieceType::Rook))],
            // Row 1: Black pawns
            vec![Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn))],
            // Rows 2-5: Empty rows
            vec![None; 8],
            vec![None; 8],
            vec![None; 8],
            vec![None; 8],
            // Row 6: White pawns
            vec![Some(Piece::White(PieceType::Pawn)),Some(Piece::White(PieceType::Pawn)),Some(Piece::White(PieceType::Pawn)),Some(Piece::White(PieceType::Pawn)),Some(Piece::White(PieceType::Pawn)),Some(Piece::White(PieceType::Pawn)),Some(Piece::White(PieceType::Pawn)),Some(Piece::White(PieceType::Pawn))],
            // Row 7: White major pieces
            vec![Some(Piece::White(PieceType::Rook)),Some(Piece::White(PieceType::Knight)),Some(Piece::White(PieceType::Bishop)),Some(Piece::White(PieceType::Queen)),Some(Piece::White(PieceType::King)),Some(Piece::White(PieceType::Bishop)),Some(Piece::White(PieceType::Knight)),Some(Piece::White(PieceType::Rook))],
        ]
    );

    let (possible_moves, set_possible_moves) = signal(vec![Move { current_pos: (0,0), new_pos: (0,0) }]);
    set_possible_moves.set(vec![]);

    let (previous_move, set_previous_move) = signal(Some(Move { current_pos: (0,0), new_pos: (0,0) }));
    set_previous_move.set(None);

    let (castle_state, set_castle_state) = signal(CastleState::new());

    view! {
        <div class="chess-container">
            <div class="chessboard">
                {move || board.get().into_iter().enumerate().map(|(row_idx, row)| {
                    view! {
                        <div class="row">
                            {row.into_iter().enumerate().map(|(col_idx, tile)| {
                                // Determine square color: alternate based on row + col indices.
                                let square_class = if (row_idx + col_idx) % 2 == 0 { "light" } else { "dark" };
                                view! {
                                    <div
                                        class={move || format!(
                                            "square {} {} {}",
                                            square_class,
                                            match selected_tile.get() {
                                                Some(selected_tile) if selected_tile.is_selected(row_idx, col_idx) => "selected",
                                                _ => ""
                                            },
                                            match possible_moves.get().iter().find(|m| m.new_pos == (row_idx, col_idx)) {
                                                Some(_) => "possible-move",
                                                None => ""
                                            }
                                        )}
                                        on:click={move |_| {
                                                let whites_turn = white_turn.get();
                                                // first check if it's a valid move
                                                match possible_moves.get().iter().find(|m| m.new_pos == (row_idx, col_idx)) {
                                                    Some(m) => {
                                                        let mut new_board = board.get();
                                                        new_board[m.new_pos.0][m.new_pos.1] = Some(new_board[m.current_pos.0][m.current_pos.1].take().unwrap());
                                                        new_board[m.current_pos.0][m.current_pos.1] = None;
                                                        set_board.set(new_board);
                                                        set_white_turn.set(!whites_turn);
                                                        set_selected_tile.set(None);
                                                        set_possible_moves.set(vec![]);
                                                        set_previous_move.set(Some(m.clone()));
                                                        // set_castle_state.set(castle_state.get().update_castle_state(m.clone()));
                                                        // console log the board
                                                        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", board.get())));
                                                        return
                                                    }
                                                    None => {}
                                                };

                                                match &board.get()[row_idx][col_idx] {
                                                    Some(piece) => {
                                                        // else try to select the piece
                                                        match piece {
                                                            Piece::White(_) if whites_turn => set_selected_tile.set(Some(SelectedTile { row: row_idx, col: col_idx })),
                                                            Piece::Black(_) if !whites_turn => set_selected_tile.set(Some(SelectedTile { row: row_idx, col: col_idx })),
                                                            _ => return
                                                        };
                                                        set_possible_moves.set(calculate_possible_moves(
                                                            row_idx,
                                                            col_idx,
                                                            &mut board.get(),
                                                            false,
                                                            whites_turn,
                                                            &previous_move.get(),
                                                            &mut castle_state.get()
                                                        ).unwrap());
                                                    }
                                                    None => {}
                                                }
                                            }
                                        }
                                    >
                                        {move || tile.as_ref().map(|piece| display_piece(piece.clone())).unwrap_or("")}
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

            .possible-move {
                background-color: rgba(0, 255, 0, 0.5);
            }
        "#}
    </style>
    }
}
