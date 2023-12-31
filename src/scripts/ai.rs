use crate::scripts::bit_board::{BitBoard, GameState};

const HEIGHT: usize = 6;
const WIDTH: usize = 7;

pub struct AIGame {
    column_order: [i64; WIDTH],
    pub debug: String
}

impl AIGame {
    pub fn new() -> Self {
            let mut column_order = [0; WIDTH];

            for i in 0..WIDTH {
                column_order[i] = WIDTH as i64 / 2 + (1 - 2 * (i as i64 % 2)) * (i as i64 + 1) / 2;
            }

            AIGame {
                column_order,
                debug: String::new(),

            }
    }

    pub fn make_move(&mut self, game: &mut BitBoard) -> Result<GameState, String> {
        let mut best_move = 0;
        let mut best_score = std::i64::MIN;
        self.debug = String::new();
        self.debug.push_str(&game.total_mask.to_string());

        if game.total_mask == 400556032 || game.total_mask == 35297673344 || game.total_mask == 42813867904 || game.total_mask == 4432806807424 || game.total_mask == 4638965237632 || game.total_mask == 5463598958464{
            return game.play_turn(5);
        }else if game.total_mask == 132136960 || game.total_mask == 35297181824 || game.total_mask == 4398111522944{
            return game.play_turn(1);
        }else if game.total_mask == 4398178633600 {
            return game.play_turn(4);
        }
        
        
        
        for col in 0..WIDTH {
            let chosen_col = self.column_order[col].try_into().unwrap(); 

            if game.is_move_valid(chosen_col) {
                if game.is_winning_move(chosen_col) {
                    return game.play_turn(chosen_col);
                }

                let init:i64 = ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2) as i64;
                game.play_move(chosen_col);
                let score = -self.negamax(game, -init, init, 13);
                let _ = game.undo_move(chosen_col);

                if score > best_score {
                    best_move = chosen_col;
                    best_score = score;
                }
            }
        }

        return game.play_turn(best_move);
    }

    pub fn negamax(&self, game: &mut BitBoard, mut alpha: i64, mut beta: i64, depth: i64) -> i64 {
        if game.get_num_moves() >= WIDTH * HEIGHT - 2 {
            return 0;
        } else if depth == 0 {
            return ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2) as i64;
        }

        for col in 0..WIDTH {
            if game.is_move_valid(col) && game.is_winning_move(col) {
                return ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2) as i64;
            }
        }
        
        let min = -(((WIDTH * HEIGHT - 2 - game.get_num_moves()) / 2) as i64);

        if alpha < min {
            alpha = min;                     
            
            if alpha >= beta {
                return alpha; 
            }
        }

        let max = (WIDTH * HEIGHT - 1 - game.get_num_moves()) as i64 / 2;
        
        if beta > max.try_into().unwrap() {
            beta = max.try_into().unwrap();
            
            if alpha >= beta {
                return beta;
            }
        }

        for col in 0..WIDTH {
            let chosen_col = self.column_order[col].try_into().unwrap(); 

            if game.is_move_valid(chosen_col) {
                game.play_move(chosen_col);
                let score = -self.negamax(game, -beta, -alpha, depth - 1);
                let _ = game.undo_move(chosen_col);

                if score >= beta {
                    return score;
                }   

                if score > alpha {
                    alpha = score;
                }
            }
        }

        return alpha;
    }
}