use eframe::egui;
use egui::{Pos2, RichText, Color32};

mod board;
use board::Board;

mod ai;
use ai::{Difficulty, Ai};

#[derive(PartialEq)]
enum PlayerPieces { X, O }
#[derive(PartialEq)]
enum Player2 { Human, Ai }

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Match 4",
        options,
        Box::new(|_cc| Box::new(Match4App::default())),
    );
}

struct Match4App {
    board: Board,
    ai: Ai,
    difficulty: Difficulty,
    game_over: bool,
    winner: char,
    started: bool,
    player_piece: PlayerPieces,
    player_2: Player2,
    player_2_turn: bool
}

impl Default for Match4App {
    fn default() -> Self {
        Self {
            board: Default::default(),
            ai: Default::default(),
            difficulty: Difficulty::Medium,
            game_over: false,
            winner: ' ',
            started: false,
            player_piece: PlayerPieces::X,
            player_2: Player2::Ai,
            player_2_turn: false
        }
    }
}

impl Match4App {
    /// Handle column [`egui::Button`] click, takes the index [`usize`] of the button relating to the column to drop a piece in
    fn handle_click(&mut self, btn: usize) {       
        // Place a piece in the column (btn), the piece to drop is player_piece for player 1 and the inverse for player 2's turn
        if !self.board.place(btn, if self.player_2_turn {if self.player_piece == PlayerPieces::X {'O'} else {'X'}} else {if self.player_piece == PlayerPieces::X {'X'} else {'O'}}) {
            return;
        }

        // If the player 2 is of type Human, toggle the turn to player 2
        // else let the Ai make a move based on the board, difficulty selected, and the inverse piece player 1 is using
        if self.player_2 == Player2::Human {
            self.player_2_turn = !self.player_2_turn;
        } else {
            self.ai.think(&mut self.board, &self.difficulty, if self.player_piece == PlayerPieces::X {'O'} else {'X'});
        }

        // If either won the game, stop it and set the winner to the char of who
        if self.board.has_won('X') || self.board.has_won('O') {
            self.game_over = true;
            self.winner = if self.board.has_won('X') {'X'} else {'O'};
        }
    }
}

#[allow(unused_must_use)]
impl eframe::App for Match4App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // If the game hasn't started, show the menu for selecting the game's options
            // else draw game
            if !self.started {
                egui::Grid::new("grid_start")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Player 1: ");
                        ui.selectable_value(&mut self.player_piece, PlayerPieces::X, "X");
                        ui.selectable_value(&mut self.player_piece, PlayerPieces::O, "O"); 
                        ui.end_row();
                        ui.label("Player 2: ");
                        ui.selectable_value(&mut self.player_2, Player2::Human, "Human");
                        ui.selectable_value(&mut self.player_2, Player2::Ai, "Ai");
                        ui.end_row();
                        let enabled = self.player_2 == Player2::Ai;
                        ui.add_enabled(enabled, egui::Label::new("Ai Difficulty: "));
                        if ui.add_enabled(enabled, egui::SelectableLabel::new(self.difficulty == Difficulty::Easy, "Easy")).clicked() {
                            self.difficulty = Difficulty::Easy;
                        }
                        if ui.add_enabled(enabled, egui::SelectableLabel::new(self.difficulty == Difficulty::Medium, "Medium")).clicked() {
                            self.difficulty = Difficulty::Easy;
                        }
                        if ui.add_enabled(enabled, egui::SelectableLabel::new(self.difficulty == Difficulty::Hard, "Hard")).clicked() {
                            self.difficulty = Difficulty::Easy;
                        }
                        ui.end_row();
                        if ui.button("Start Game").clicked() {
                            self.started = true;
                        }
                    }); 
            } else {
                // If the game is over, make a moveable window display the game is over and who won
                if self.game_over {
                    egui::Window::new("Game Over!").collapsible(false).resizable(false).default_width(60.).default_height(50.).default_pos(Pos2{x:_frame.info().window_info.size.x/2.-60.,y:_frame.info().window_info.size.y/2.-25.}).show(ctx, |ui| {
                        ui.label(format!("{} Wins!", self.winner.to_string()).as_str());
                    });
                }
                // Disable the ui for elements below
                ui.set_enabled(!self.game_over);

                // A grid where the top row are the buttons and the remainer are labels of the board cells 
                egui::Grid::new("grid").striped(true).show(ui, |ui| {
                    for i in 0..7 {
                        // Create a button, pad the size to fit, set text to index plus one, when clicked call handle_click with the index
                        if ui.add_sized([_frame.info().window_info.size.x/7.6, 50.], egui::Button::new((i+1).to_string())).clicked() {
                            self.handle_click(i);
                        }
                    }
                    ui.end_row();

                    // Create the lables used to display the cells in the board
                    for row in 0..6 {
                        for column in 0..7 {
                            // If the cell is 'X' it is drawn green, 'O' is light red
                            ui.add_sized([_frame.info().window_info.size.x/7.6, _frame.info().window_info.size.x/6.-50.], egui::Label::new(RichText::new(self.board[(row, column)].to_string()).color(if self.board[(row, column)] == 'X' {Color32::GREEN} else {Color32::LIGHT_RED})));
                        }
                        ui.end_row();
                    }
                });
            }
        });
    }
}
