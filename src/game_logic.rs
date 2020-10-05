extern crate rand;
use rand::Rng;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug, Clone, Copy)]
struct Tile {
    current_amount: u32,
    changed: bool,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {current_amount: 0, changed: false}
    }
}
#[derive(Debug, Clone)]
pub struct Game {
    pub score: u32,
    board: Vec<Vec<Tile>>,
    empty_tiles: Vec<(usize, usize)>,
}

pub fn draw_game (a_game: &Game) {
    let dim = a_game.board.len();
    for i in 0..dim {
        let mut row = String::from("");
        for j in 0..dim {
            let curamt = a_game.board[i][j].current_amount.to_string();
            row = row + " " + &curamt;
        }
        println!("{}", row);
    }
    println!("----");
}

impl Game {
    pub fn new_game(x: usize) -> Game {
        let mut temp_board = Vec::new();
        let mut temp_empty_tiles = Vec::new();
        for i in 0..x {
            let mut inner_temp_board = Vec::new();
            for j in 0..x {
                inner_temp_board.push(Tile::default());
                temp_empty_tiles.push((i, j));
            }
            temp_board.push(inner_temp_board);
        }
        let mut temp_game = Game {
            score: 0,
            board: temp_board,
            empty_tiles: temp_empty_tiles,
        };
        temp_game.add_tile();
        temp_game
    }

    pub fn validate_and_execute_next_state(a_game: Game, dir: &Direction) -> Game {
        let dim = a_game.board.len();
        let mut differs = false;
        let current_state = a_game;
        let mut next_state = current_state.clone();
        next_state.player_move(dir);

        for i in 0..dim {
            for j in 0..dim {
                if current_state.board[i][j].current_amount != next_state.board[i][j].current_amount {
                    differs = true;
                }
            }
        }
        if differs {
            next_state.add_tile();
            next_state
        } else {
            current_state
        }
    }

    pub fn game_over(&self) -> bool {
        let x = &self.board.len() - 1;
        if self.empty_tiles.len() != 0 {
            return false
        }
        for i in 0..x {
            for j in 0..x {
                if self.board[i][j].current_amount == self.board[i][j+1].current_amount || self.board[i][j].current_amount == self.board[i+1][j].current_amount {
                    return false;
                }
            }
        }
        return true;
    }

    fn player_move(&mut self, dir: &Direction) {
        let x = &self.board.len();
        for i in 0..*x {
            for j in 0..*x {
                let outer_x = match dir {
                    Direction::Left => i,
                    Direction::Right => i,
                    Direction::Down => x-1-j,
                    Direction::Up => j,
                };
                let outer_y = match dir {
                    Direction::Left => j,
                    Direction::Right => x-1-j,
                    Direction::Down => i,
                    Direction::Up => i,
                };
                self.board[outer_x][outer_y].changed = false;
                if self.board[outer_x][outer_y].current_amount != 0 && j > 0 {
                    for k in 0..j {
                        let inner_x = match dir {
                            Direction::Left => i,
                            Direction::Right => i,
                            Direction::Down => x-1-k,
                            Direction::Up => k,
                        };
                        let inner_y = match dir {
                            Direction::Left => k,
                            Direction::Right => x-1-k,
                            Direction::Down => i,
                            Direction::Up => i,
                        };
                        if !self.board[inner_x][inner_y].changed && self.board[outer_x][outer_y].current_amount == self.board[inner_x][inner_y].current_amount {
                            self.board[inner_x][inner_y].current_amount = self.board[inner_x][inner_y].current_amount + self.board[outer_x][outer_y].current_amount;
                            self.board[inner_x][inner_y].changed = true;
                            self.board[outer_x][outer_y].current_amount = 0;
                            self.add_empty_tile(outer_x, outer_y);
                            break
                        } else if self.board[inner_x][inner_y].current_amount == 0 {
                            self.board[inner_x][inner_y].current_amount = self.board[outer_x][outer_y].current_amount;
                            self.board[outer_x][outer_y].current_amount = 0;
                            self.add_empty_tile(outer_x, outer_y);
                            self.remove_empty_tile(inner_x, inner_y);
                            break
                        }
                    }
                }
            }
        }
    }
    pub fn add_tile(&mut self) {
        let number_of_empty = self.empty_tiles.len();
        let rand_no = rand::thread_rng().gen_range(0, number_of_empty);
        let value = if rand::thread_rng().gen_bool(1.0 / 10.0) {4} else {2};
        let x_and_y = self.empty_tiles[rand_no];
        self.remove_empty_tile(x_and_y.0, x_and_y.1);
        self.board[x_and_y.0][x_and_y.1].current_amount = value;

    }
    fn remove_empty_tile(&mut self, x: usize, y: usize) {
        for i in 0..self.empty_tiles.len() {
            if self.empty_tiles[i].0 == x && self.empty_tiles[i].1 == y {
                self.empty_tiles.swap_remove(i);
                break;
            }
        }
    }
    fn add_empty_tile(&mut self, x: usize, y: usize) {
        self.empty_tiles.push((x, y))
    }
}