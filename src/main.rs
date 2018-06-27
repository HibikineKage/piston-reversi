extern crate arrayvec;
extern crate vecmath;

extern crate piston_window;

use arrayvec::ArrayVec;
use piston_window::*;
use vecmath::*;

const HEIGHT: u32 = 480u32;
const WIDTH: u32 = 640u32;
const LINE_WIDTH: f64 = 2.0;
const BOARD_WIDTH: usize = 8;
const CELL_WIDTH: f64 = HEIGHT as f64 / BOARD_WIDTH as f64;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const CELL_SPACE: f64 = CELL_WIDTH / 16.0;
#[derive(Clone, Copy, PartialEq, Debug)]
enum Cell {
    None,
    Black,
    White,
}
struct Board {
    pub cells: [Cell; 64],
}

const DIRECTIONS: [Vector2<isize>; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];
fn in_available_range(position: [isize; 2]) -> bool {
    0 <= position[0]
        && position[0] < BOARD_WIDTH as isize
        && 0 <= position[1]
        && position[1] < BOARD_WIDTH as isize
}
impl Board {
    pub fn new() -> Self {
        let mut cells = [Cell::None; 64];
        cells[3 + BOARD_WIDTH * 3] = Cell::Black;
        cells[3 + BOARD_WIDTH * 4] = Cell::White;
        cells[4 + BOARD_WIDTH * 3] = Cell::White;
        cells[4 + BOARD_WIDTH * 4] = Cell::Black;
        Self { cells: cells }
    }
    pub fn position_to_cell(&self, position: Vector2<isize>) -> Option<Cell> {
        if !in_available_range(position) {
            return None;
        }
        Some(self.cells[position[0] as usize + position[1] as usize * BOARD_WIDTH])
    }
    pub fn put(&mut self, position: [usize; 2], color: &Cell) {
        self.cells[position[0] + position[1] * BOARD_WIDTH] = *color;
        let mut count = 0;
        for i in DIRECTIONS
            .iter()
            .zip(self.puttable_directions(position, color).iter())
        {
            let mut current_position = [position[0] as isize, position[1] as isize];
            for _ in 0..*i.1 {
                current_position = vec2_add(current_position, *i.0);
                self.cells
                    [current_position[0] as usize + current_position[1] as usize * BOARD_WIDTH] =
                    *color;
            }
        }
    }
    fn puttable_directions(&self, position: [usize; 2], color: &Cell) -> [u32; 8] {
        let directions: ArrayVec<[u32; 8]> = ArrayVec::from(DIRECTIONS)
            .iter()
            .map(|direction| {
                let mut current_position = [position[0] as isize, position[1] as isize];
                current_position = vec2_add(current_position, *direction);
                let cell = self
                    .position_to_cell(current_position)
                    .unwrap_or(Cell::None);
                match cell {
                    Cell::None => return 0u32,
                    other_color if *color == other_color => return 0u32,
                    _ => (),
                };
                let mut count = 1u32;
                while {
                    current_position = vec2_add(current_position, *direction);
                    match self
                        .position_to_cell(current_position)
                        .unwrap_or(Cell::None)
                    {
                        Cell::None => return 0u32,
                        other_color if *color != other_color => true,
                        _ => false,
                    }
                } {
                    count += 1;
                }
                count
            })
            .collect();
        directions.into_inner().unwrap()
    }
    pub fn puttable(&self, position: [usize; 2], color: &Cell) -> bool {
        if let Some(cell) = self.position_to_cell([position[0] as isize, position[1] as isize]) {
            if cell != Cell::None {
                return false;
            }
        } else {
            return false;
        }
        self.puttable_directions(position, color)
            .iter()
            .find(|value| **value > 0u32)
            .is_some()
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Reversi", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut select_pos = [0usize; 2];
    let mut board = Board::new();
    let mut current_color = Cell::Black;
    while let Some(event) = window.next() {
        if let Some(mouse) = event.mouse_cursor_args() {
            let x_cell = (mouse[0] * 8.0 / HEIGHT as f64).floor() as usize;
            let y_cell = (mouse[1] * 8.0 / HEIGHT as f64).floor() as usize;
            if x_cell < 8 && y_cell < 8 {
                select_pos = [x_cell, y_cell];
            }
        }
        if let Some(_mouse) = event.press_args() {
            if board.puttable(select_pos, &current_color) {
                board.put(select_pos, &current_color);
                current_color = match current_color {
                    Cell::Black => Cell::White,
                    Cell::White => Cell::Black,
                    Cell::None => Cell::Black,
                }
            }
        }
        window.draw_2d(&event, |context, graphics| {
            clear([0.3, 1.0, 0.3, 1.0], graphics);
            // Mouse Selecting
            rectangle(
                [0.6, 1.0, 0.6, 1.0],
                [
                    select_pos[0] as f64 * CELL_WIDTH,
                    select_pos[1] as f64 * CELL_WIDTH,
                    CELL_WIDTH,
                    CELL_WIDTH,
                ],
                context.transform,
                graphics,
            );
            // draw lines
            for i in 0..9 {
                rectangle(
                    BLACK,
                    [i as f64 * CELL_WIDTH, 0.0, LINE_WIDTH, HEIGHT as f64],
                    context.transform,
                    graphics,
                );
                rectangle(
                    BLACK,
                    [0.0, i as f64 * CELL_WIDTH, HEIGHT as f64, LINE_WIDTH],
                    context.transform,
                    graphics,
                );
            }
            // draw board
            for i in 0..64 {
                match board.cells[i] {
                    Cell::Black => Some([0.0f32, 0.0, 0.0, 1.0]),
                    Cell::White => Some([1.0f32, 1.0, 1.0, 1.0]),
                    Cell::None => None,
                }.and_then(|color: [f32; 4]| {
                    ellipse(
                        color,
                        [
                            (i % 8) as f64 * HEIGHT as f64 / 8.0 + CELL_SPACE,
                            (i / 8) as f64 * HEIGHT as f64 / 8.0 + CELL_SPACE,
                            HEIGHT as f64 / 8.0 - CELL_SPACE * 2.0,
                            HEIGHT as f64 / 8.0 - CELL_SPACE * 2.0,
                        ],
                        context.transform,
                        graphics,
                    );
                    Some(color)
                });
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use Board;
    use Cell;
    use BOARD_WIDTH;
    #[test]
    fn test_puttable() {
        let board = Board::new();
        assert_eq!(board.puttable([3, 3], &Cell::Black), false);
        assert_eq!(board.puttable([3, 2], &Cell::Black), false);
        assert_eq!(board.puttable([3, 2], &Cell::White), true);
        assert_eq!(board.puttable([4, 2], &Cell::Black), true);
    }
    #[test]
    fn test_put() {
        let mut board = Board::new();
        board.put([3, 2], &Cell::White);
        assert_eq!(board.cells[3 + 2 * BOARD_WIDTH], Cell::White);
        assert_eq!(board.cells[3 + 3 * BOARD_WIDTH], Cell::White);
        board.put([2, 2], &Cell::Black);
        assert_eq!(board.cells[2 + 2 * BOARD_WIDTH], Cell::Black);
        assert_eq!(board.cells[3 + 3 * BOARD_WIDTH], Cell::Black);
    }
}
