#![feature(globs)]

extern crate vecmath;

extern crate piston_window;

use piston_window::*;
use vecmath::Vector2;

const HEIGHT: u32 = 480u32;
const WIDTH: u32 = 640u32;
const LINE_WIDTH: f64 = 2.0;
const BOARD_WIDTH: usize = 8;
const CELL_WIDTH: f64 = HEIGHT as f64 / BOARD_WIDTH as f64;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const CELL_SPACE: f64 = CELL_WIDTH / 16.0;
#[derive(Clone, Copy)]
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
    [1, -1],
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
    pub fn puttable(&self, position: [usize; 2]) -> bool {
        DIRECTIONS.iter().map(|direction| {
            let mut current_position = [position[0] as isize, position[1] as isize];
            current_position[0] += direction[0];
            current_position[1] += direction[1];
            if !in_available_range(current_position) {
                return false;
            }
            true
        });
        false
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Reversi", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut select_pos = [0.0f64; 2];
    let mut board = Board::new();
    let mut current_color = Cell::Black;
    while let Some(event) = window.next() {
        if let Some(mouse) = event.mouse_cursor_args() {
            let x_cell = (mouse[0] * 8.0 / HEIGHT as f64).floor();
            let y_cell = (mouse[1] * 8.0 / HEIGHT as f64).floor();
            if 0.0 <= x_cell && x_cell < 8.0 && 0.0 <= y_cell && y_cell < 8.0 {
                select_pos = [x_cell, y_cell];
            }
        }
        if let Some(_mouse) = event.press_args() {
            board.cells[(select_pos[0] + select_pos[1] * BOARD_WIDTH as f64) as usize] =
                current_color;
            current_color = match current_color {
                Cell::Black => Cell::White,
                Cell::White => Cell::Black,
                Cell::None => Cell::Black,
            }
        }
        window.draw_2d(&event, |context, graphics| {
            clear([0.3, 1.0, 0.3, 1.0], graphics);
            // Mouse Selecting
            rectangle(
                [0.6, 1.0, 0.6, 1.0],
                [
                    select_pos[0] * CELL_WIDTH,
                    select_pos[1] * CELL_WIDTH,
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
