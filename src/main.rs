extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use std::collections::HashSet;
use rand::Rng;

use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food
}

impl Game {
    fn render (&mut self, arg: &RenderArgs) {
        use graphics;

        let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);

        });

        self.snake.render(&mut self.gl, arg);

        self.food.render(&mut self.gl, arg);

        
    }

    fn check_for_collision(&mut self) {
        let mut collision_detected = false;
        let mut snake_piece_positions: HashSet<&(i32, i32)> = HashSet::new();

        self.snake.body.iter().for_each(|pos| {
            snake_piece_positions.insert(pos);

            if self.food.pos_x == pos.0 && self.food.pos_y == pos.1 {
                collision_detected = true;
            }
        });

        

        if snake_piece_positions.len() != self.snake.body.len() {
            self.end_game();
        }

        if collision_detected {
            self.collision();
        }
        
    }

    fn end_game(&mut self) {

    }

    fn collision(&mut self) {

        // not proud of this collision detection
        let last_node = self.snake.body.pop_back().unwrap();
        self.snake.body.push_back(last_node);

        let mut new_node = last_node.clone();
        new_node.1 += 1;
        self.snake.body.push_back(new_node);

        let mut rng = rand::thread_rng();
        self.food = Food { pos_x: rng.gen_range(0, 400 / 20), pos_y: rng.gen_range(0, 400 / 20) }
    }

    fn update(&mut self) {
        self.snake.update();

        self.check_for_collision();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        };
    }
}

struct Food {
    pos_x: i32,
    pos_y: i32
}

impl Food {
    fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs) {

        use graphics;

        let BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let food = graphics::rectangle::square((self.pos_x * 20) as f64, (self.pos_y * 20) as f64, 20_f64);

        gl.draw(arg.viewport(), |c, gl| {

            let transform = c.transform;
            graphics::rectangle(BLUE, food, transform, gl);

        })
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs) {

        use graphics;

        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                return graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64);
            }).collect();

        gl.draw(arg.viewport(), |c, gl| {

            let transform = c.transform;

            squares.into_iter()
                .for_each(|square| {
                    graphics::rectangle(RED, square, transform, gl);
                })
        })      
    
    }
    
    fn update(&mut self) {
        let mut new_body = (*self.body.front().expect("Snake has no body")).clone();
        match self.dir {
            Direction::Left => new_body.0 -= 1,
            Direction::Right => new_body.0 += 1,
            Direction::Up => new_body.1 -= 1,
            Direction::Down => new_body.1 += 1,
        }

        self.body.push_front(new_body);

        self.body.pop_back().unwrap();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake",
        [400, 400]
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut rng = rand::thread_rng();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            dir: Direction::Right,
            body: LinkedList::from_iter((vec![(0,0), (0, 1)]).into_iter())
        },
        food: Food { pos_x: rng.gen_range(0, 400 / 20), pos_y: rng.gen_range(0, 400 / 20) }
    };


    let mut events = Events::new(EventSettings::new().ups(8));

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            game.pressed(&k.button);
        }
    }
}
