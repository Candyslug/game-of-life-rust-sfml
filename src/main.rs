/* Conway's game of life, using SFML & Rust */
/* Made for learning purposes! */

extern crate sfml;
use std::ops::{Index, IndexMut};

use sfml::system::{Vector2f, Vector2i};
use sfml::window::{ContextSettings, VideoMode, Event, Style};
use sfml::graphics::{RenderWindow, RenderTarget, Shape, Color, Transformable, RectangleShape};

struct Cell {
    x_pos: i32,
    y_pos: i32,
    active: bool,
    neighbour_count: i32,
}

struct Grid {
    x_cell_count: i32,
    y_cell_count: i32,
    cells: Vec<Cell>,
    running_sim: bool,
}

impl Grid {
    pub fn new(x_size: i32, y_size: i32) -> Grid {
        let mut cell_list = Vec::new();
        for x in 0..x_size {
            for y in 0..y_size {
                cell_list.push(Cell {
                    x_pos: x,
                    y_pos: y,
                    active: rand::random(),
                    neighbour_count: 0,
                });
            }
        }
        Grid {
            x_cell_count: x_size,
            y_cell_count: y_size,
            cells: cell_list,
            running_sim: false,
        }
    }
                        
    pub fn draw_grid_lines(&mut self, window: &mut RenderWindow) {
        for x in 0..self.x_cell_count {
            let mut vertical_line = RectangleShape::new();
            vertical_line.set_position(Vector2f::new(
                    -1.0 + ((window.size().x as f32) / self.x_cell_count as f32) * (x as f32),
                    0.0));
            vertical_line.set_size(Vector2f::new(2.0, window.size().y as f32));
            vertical_line.set_fill_color(Color::rgba(0, 0, 0, 255));
            window.draw(&vertical_line);
        }

        for y in 0..self.y_cell_count {
            let mut horizontal_line = RectangleShape::new();
            horizontal_line.set_position(Vector2f::new(
                    0.0,
                    -1.0 + ((window.size().y as f32) / self.y_cell_count as f32) * (y as f32)));
            horizontal_line.set_size(Vector2f::new(window.size().x as f32, 2.0));
            horizontal_line.set_fill_color(Color::rgba(0, 0, 0, 255));
            window.draw(&horizontal_line);
        }
    }

    pub fn draw_cells(&mut self, window: &mut RenderWindow) {
        for i in &self.cells {
            if i.active {
                let mut cell_square = RectangleShape::new();
                cell_square.set_position(Vector2f::new(
                        ((window.size().x as f32) / self.x_cell_count as f32) * (i.x_pos as f32),
                        ((window.size().y as f32) / self.y_cell_count as f32) * (i.y_pos as f32)));
                cell_square.set_size(Vector2f::new(
                        (window.size().x as f32) / self.x_cell_count as f32,
                        (window.size().y as f32) / self.y_cell_count as f32));
                cell_square.set_fill_color(Color::rgba(130, 50, 50, 255));
                window.draw(&cell_square);
            }
        }
    }

    pub fn get_cell_active(&mut self, x_pos: i32, y_pos: i32) -> bool {
        let index = (self.x_cell_count * (y_pos - 1)) + (x_pos - 1);
        return self.cells.as_mut_slice().index(index as usize).active;
    }

    pub fn update_cells(&mut self) {
        for i in &mut self.cells {
            // Left cells
            let left_cell = Vector2i::new(i.x_pos - 1, i.y_pos);
            let left_down_cell = Vector2i::new(i.x_pos - 1, i.y_pos + 1);
            let left_up_cell = Vector2i::new(i.x_pos - 1, i.y_pos - 1);
            let up = Vector2i::new(i.x_pos, i.y_pos - 1);
            let down = Vector2i::new(i.x_pos, i.y_pos + 1);
            let right_cell = Vector2i::new(i.x_pos + 1, i.y_pos);
            let right_down_cell = Vector2i::new(i.x_pos + 1, i.y_pos + 1);
            let right_up_cell = Vector2i::new(i.x_pos + 1, i.y_pos - 1);

            if i.x_pos >= 1 {
                // Left
            }
        }
    }
}

fn main() {
    let desktop = VideoMode::desktop_mode();
    let mut window = RenderWindow::new(VideoMode::new(800, 700, 32),
                            "Conway's Game of Life - Made with Rust and SFML",
                            Style::CLOSE,
                            &ContextSettings::default());
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);
    window.set_position(Vector2i::new(
            (desktop.width as i32 / 2) - (window.size().x as i32 / 2),
            (desktop.height as i32 / 2) - (window.size().y as i32 / 2)));


    let mut grid = Grid::new(10, 10);

    while window.is_open() {
        for event in window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed {
                    code: sfml::window::Key::ESCAPE, ..
                } => {
                    window.close();
                },
                Event::KeyPressed {
                    code: sfml::window::Key::SPACE, ..
                } => {
                    grid.running_sim = !grid.running_sim;
                },

                _ => {},
            }
        }
   
        window.clear(Color::rgba(100, 100, 100, 100));
        grid.draw_cells(&mut window);
        grid.draw_grid_lines(&mut window);
        window.display();
    } 
}
