use super::automata::CellularGrid;

/// Graphics lib
use piston_window::{
    PistonWindow,
    WindowSettings,
    clear,
    rectangle,
};

/// Visualizer for sim
pub struct Window {
    window_width: u32,
    window_height: u32,

    cell_width: f32,
    cell_height: f32,

    sim: CellularGrid,

    title: String,
}

impl Window {
    /// Creates a new grid visualizer
    pub fn new(width: u32, height: u32, sim: CellularGrid, title: String) -> Self {
        let sim_size = sim.size();

        let cell_width = width as f32 / sim_size.0 as f32;
        let cell_height = height as f32 / sim_size.1 as f32;

        Self {
            window_width: width,
            window_height: height,
            cell_width,
            cell_height,
            sim,
            title,
        }
    }
}

impl Window {
    /// Starts the windows loop
    pub fn run(&mut self) {
        let mut window: PistonWindow = 
            WindowSettings::new(self.title.as_str(), [self.window_width, self.window_height])
            .exit_on_esc(true).build().expect("Failed to build window");

        let (size_x, size_y) = self.sim.size();

        while let Some(e) = window.next() {
            //self.sim.update();

            window.draw_2d(&e, |c, g, _device| {
                // Clear screen
                clear([1.0; 4], g);

                // Drawing
                let grid = self.sim.grid_as_ref();
                for x in 0..size_x {
                    for y in 0..size_y {
                        let color = grid[y * size_x + x].channels[0] as f32;
                        rectangle(
                            [color, color, color, 1.],
                            [x as f64 * self.cell_width as f64, y as f64 * self.cell_height as f64, self.cell_width as f64, self.cell_height as f64],
                            c.transform, 
                            g
                        )
                    }
                }
            });
        }
    }
}
