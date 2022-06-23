extern crate rand;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use graphics::rectangle;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use rand::Rng;

use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, arg);
        self.food.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update();
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

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x,y)| {
                graphics::rectangle::square(
                    (x * 20) as f64, 
                    (y * 20) as f64, 
                    20_f64
                )
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        });
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);

        self.body.pop_back().unwrap();
    }
}

struct Food {
    x: i32,
    y: i32,
}

impl Food {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const WHITE: [f32; 4] = [1.0; 4];

        gl.draw(args.viewport(), |c, gl| {
            rectangle(
                WHITE, 
                rectangle::square((self.x * 20) as f64, (self.y * 20) as f64, 20_f64), 
                graphics::Transformed::trans(c.transform, 0.0, 0.0),
                gl
            );
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut rng = rand::thread_rng();

    let mut window: GlutinWindow = WindowSettings::new("snake game", [400, 400])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            dir: Direction::Right,
        },
        food: Food { 
            x: rng.gen_range(0..20), 
            y: rng.gen_range(0..20),
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(_u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            game.pressed(&k.button);
        }
    }
}
