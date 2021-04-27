/* Conway's game of life, using SFML & Rust */
/* Made for learning purposes! */

extern crate sfml;
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, Event, Style};
use sfml::graphics::{RenderWindow, RenderTarget, Shape, Color, Transformable, RectangleShape};

struct Cell {
    x_pos: i32,
    y_pos: i32,
    active: bool,
}

struct Grid {
    x_cell_count: i32,
    y_cell_count: i32,
    cells: Vec<Cell>,
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
                });
            }
        }
        Grid {
            x_cell_count: x_size,
            y_cell_count: y_size,
            cells: cell_list,
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
}

fn main() {
    
    let mut window = RenderWindow::new(VideoMode::new(800, 700, 32),
                            "Hello world",
                            Style::CLOSE,
                            &ContextSettings::default());

    let mut grid = Grid::new(20, 20);

    while window.is_open() {
        for event in window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed {
                    code: sfml::window::Key::ESCAPE, 
                    alt: false, ctrl: false, shift: false, system: false
                } => {
                    window.close();
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
