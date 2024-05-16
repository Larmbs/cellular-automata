
// Model responsible for running sim
mod automata;
// Helps display the sim
mod display;


fn main() {
    let grid = automata::CellularGrid::new_with_noise(200, 150);
    let mut display = display::Window::new(600, 400, grid, String::from("Sim"));
    display.run();
}
