extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::cell::RefMut;
use std::cell::RefCell;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::collections::VecDeque;
use std::rc::{Rc};

const CELL_SIZE: f64 = 10.0;

#[derive(Copy, Clone)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct ChainLink {
    x: f64,
    y: f64,
}

impl ChainLink {
    fn new(x: f64, y: f64) -> ChainLink {
        ChainLink { x, y }
    }
}

#[derive(Clone)]
struct Snake{
    direction: Direction,
    chain: VecDeque<ChainLink>,
    growth: i64,
    poop: i64,
    app_state: Rc<RefCell<AppState>>
}

impl Snake {
    fn new(app_state: Rc<RefCell<AppState>>) -> Snake {
        let snake_chain = ChainLink::new(0.0, 0.0);
        let mut chain: VecDeque<ChainLink> = VecDeque::new();
        chain.push_back(snake_chain);

        Snake {
            chain,
            direction: Direction::None,
            growth: 0,
            poop: 0,
            app_state
        }
    }

    fn new_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn make_step(&mut self, step_size: f64) {
        const STEP_MULTIPLICATOR: f64 = 50.0;

        let old_head = self.chain.front();
        if let Some(old_head_ref) = old_head {
            let mut new_y = old_head_ref.y;
            let mut new_x = old_head_ref.x;

            match self.direction {
                Direction::Up => new_y -= step_size * STEP_MULTIPLICATOR,
                Direction::Down => new_y += step_size * STEP_MULTIPLICATOR,
                Direction::Left => new_x -= step_size * STEP_MULTIPLICATOR,
                Direction::Right => new_x += step_size * STEP_MULTIPLICATOR,
                _ => {}
            }

            self.chain.push_front(ChainLink::new(new_x, new_y));

            if self.growth == 0 && self.poop == 0 {
                self.chain.pop_back();
            } else if self.growth > 0 {
                self.growth -= 1;
            } else if self.poop > 0 {
                self.poop -= 1;
                let tail = self.chain.pop_back();
                if let Some(tail_ref) = tail{
                    let mut app_state: RefMut<_> = self.app_state.borrow_mut();
                    app_state.add_item_to_map(tail_ref);
                }
            }
        }
    }
}

struct App {
    gl: GlGraphics,
    snake: Snake,
    app_state: Rc<RefCell<AppState>>
}

#[derive(Clone)]
struct AppState {
    all_items_on_map: Vec<ChainLink>
}

impl AppState {
    fn new() -> AppState{
        AppState {
            all_items_on_map: Vec::new()
        }
    }

    fn add_item_to_map(&mut self, item:ChainLink){
        self.all_items_on_map.push(item);
    }
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        let rect: [f64; 4] = rectangle::square(0.0, 0.0, 10.0);

        let snake = &self.snake;
        let app_state = &self.app_state;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            
            for snake_item in snake.chain.iter(){
                let transform = c
                .transform
                .trans(snake_item.x, snake_item.y)
                .trans(CELL_SIZE / 2.0, CELL_SIZE / 2.0);

                rectangle(BLUE, rect, transform, gl);
            } 

            for item in app_state.borrow().all_items_on_map.iter(){
                let transform = c
                .transform
                .trans(item.x, item.y)
                .trans(CELL_SIZE / 2.0, CELL_SIZE / 2.0);

                rectangle(RED, rect, transform, gl);                
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.snake.make_step(args.dt);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("snake", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let app_state = Rc::new(RefCell::new(AppState::new()));
    let snake = Snake::new(app_state.clone());

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake,
        app_state
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Up) => app.snake.new_direction(Direction::Up),
                Button::Keyboard(Key::Down) => app.snake.new_direction(Direction::Down),
                Button::Keyboard(Key::Left) => app.snake.new_direction(Direction::Left),
                Button::Keyboard(Key::Right) => app.snake.new_direction(Direction::Right),
                Button::Keyboard(Key::Space) => app.snake.new_direction(Direction::None),
                Button::Keyboard(Key::Return) => app.snake.poop += 1,
                _ => {}
            }
        }
    }
}
