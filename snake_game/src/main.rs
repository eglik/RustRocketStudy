extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

use rand::prelude::*;

const size: i32 = 8;

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}

struct Food {
    x: i32,
    y: i32,
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
    is_live: bool,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;

        let BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let food_color: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        let text_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let food = graphics::rectangle::square((self.food.x * size) as f64, (self.food.y * size) as f64, size as f64);

        self.gl.draw(arg.viewport(), |_c, gl | {
            let transform = _c.transform;

            graphics::clear(BACKGROUND, gl);
            graphics::rectangle(food_color, food, transform, gl);
        });

        self.snake.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        if self.is_live {
            self.snake.update(&mut self.food, &mut self.is_live);
        }
    }

    fn pressed(&mut self, button: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match button {
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
    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs) {
        use graphics;
        let snake_color: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
        

        let squares: Vec<graphics::types::Rectangle> = self.body.iter().map(|&(x, y)| {
            graphics::rectangle::square((x * size) as f64, (y * size) as f64, size as f64)
        }).collect();
        
        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter().for_each(|square| graphics::rectangle(snake_color, square, transform, gl));
        });
    }

    fn update(&mut self, food: &mut Food, is_live: &mut bool) {
        let mut new_head = (*self.body.front().expect("snake is headless")).clone();

        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Down => new_head.1 += 1,
            Direction::Up =>  new_head.1 -= 1,
        }

        if food.x == new_head.0 && food.y == new_head.1 {
            self.body.push_back(*self.body.back().unwrap());

            // food generater loop
            'food: loop {
                let mut is_possible: bool = true;

                let food_x: i32 = rand::thread_rng().gen_range(0, 24);
                let food_y: i32 = rand::thread_rng().gen_range(0, 24);

                food.x = food_x;
                food.y = food_y;

                for i in self.body.iter() {
                    if i.0 == food_x && i.1 == food_y {
                        is_possible = false;
                    }
                }

                if is_possible
                {
                    break 'food;
                }
            }
        }

        for i in self.body.iter() {
            if i.0 == new_head.0 && i.1 == new_head.1 {
                *is_live = false;
                self.dir = Direction::Right;
            }
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Sname Game", [200, 200])
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake { body: LinkedList::from_iter((vec![(0,0), (0,1)]).into_iter()), dir: Direction::Right },
        food: Food { x: 4, y: 4 },
        is_live: true,
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }

        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update();
        }
    }
}