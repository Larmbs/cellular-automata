mod grid;
use macroquad::prelude::*;

#[macroquad::main("Grid Example")]
async fn main() {
    let mut grid = grid::Grid::new(200, 150, 800, 600);
    grid.gen_random(300);

    loop {
        clear_background(BLACK);

        grid.update();

        grid.display();

        next_frame().await;
    }
}