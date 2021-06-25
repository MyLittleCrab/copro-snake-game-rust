extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::EventLoop;
use std::cell::RefCell;
// use std::cell::RefMut;
use std::collections::VecDeque;
use std::rc::Rc;

const HALF_CELL_SIZE: f64 = 5.0;
const STEP_MULTIPLICATOR: f64 = 100.0;
const GROW_MULTIPLICATOR: i64 = 4;
const INITIAL_SIZE: usize = 25;

#[derive(Copy, Clone, Debug)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
enum ChainType {
    Snake,
    Poop,
    // Wall
}

#[derive(Copy, Clone, Debug)]
struct ChainLink {
    x: f64,
    y: f64,
    t: ChainType,
}

impl ChainLink {
    fn new(x: f64, y: f64, t: ChainType) -> ChainLink {
        ChainLink { x, y, t }
    }

    fn intersects(&self, other: &ChainLink) -> bool {
        let this_x1 = self.x - HALF_CELL_SIZE;
        let this_x2 = self.x + HALF_CELL_SIZE;
        let this_y1 = self.y - HALF_CELL_SIZE;
        let this_y2 = self.y + HALF_CELL_SIZE;
        let other_x1 = other.x - HALF_CELL_SIZE;
        let other_x2 = other.x + HALF_CELL_SIZE;
        let other_y1 = other.y - HALF_CELL_SIZE;
        let other_y2 = other.y + HALF_CELL_SIZE;

        if (other_x1 <= this_x2 && other_x1 >= this_x1
            || other_x2 <= this_x2 && other_x2 >= this_x1)
            && (other_y1 <= this_y2 && other_y1 >= this_y1
                || other_y2 <= this_y2 && other_y2 >= this_y1)
        {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Clone)]
struct Snake {
    direction: Direction,
    chain: VecDeque<ChainLink>,
    growth: i64,
    poop: i64,
    app_state: Rc<RefCell<AppState>>,
}

impl Snake {
    fn new(app_state: Rc<RefCell<AppState>>) -> Snake {
        let snake_chain = ChainLink::new(0.0, 0.0, ChainType::Snake);
        let mut chain: VecDeque<ChainLink> = VecDeque::with_capacity(INITIAL_SIZE * 4);

        for _ in 0..INITIAL_SIZE {
            chain.push_back(snake_chain);
        }

        Snake {
            chain,
            direction: Direction::None,
            growth: 0,
            poop: 0,
            app_state,
        }
    }

    fn new_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn make_step(&mut self, step_size: f64) {
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

            self.chain
                .push_front(ChainLink::new(new_x, new_y, ChainType::Snake));

            self.find_something_to_eat();

            if self.growth == 0 && self.poop == 0 {
                self.chain.pop_back();
            } else if self.growth > 0 {
                self.growth -= 1;
            } else if self.poop > 0 {
                self.poop -= 1;
                let tail = self.chain.pop_back();
                if let Some(mut tail_ref) = tail {
                    let mut app_state = self.app_state.borrow_mut();
                    tail_ref.t = ChainType::Poop;
                    app_state.add_item_to_map(tail_ref);
                }
            }
        }
    }

    fn find_something_to_eat(&mut self) {
        let head = self.chain.front();
        if let Some(head_ref) = head {
            let mut app_state = self.app_state.borrow_mut();

            if let Some(index) = app_state
                .all_items_on_map
                .iter()
                .position(|item| head_ref.intersects(item))
            {
                self.growth += GROW_MULTIPLICATOR;
                app_state.all_items_on_map.remove(index);
            }
        }
    }
}

struct App {
    gl: GlGraphics,
    snake: Snake,
    app_state: Rc<RefCell<AppState>>,
}

#[derive(Clone)]
struct AppState {
    all_items_on_map: Vec<ChainLink>,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            all_items_on_map: Vec::new(),
        }
    }

    fn add_item_to_map(&mut self, item: ChainLink) {
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

            for snake_item in snake.chain.iter() {
                let transform = c
                    .transform
                    .trans(snake_item.x, snake_item.y)
                    .trans(HALF_CELL_SIZE, HALF_CELL_SIZE);

                rectangle(BLUE, rect, transform, gl);
            }

            for item in app_state.borrow().all_items_on_map.iter() {
                let transform = c
                    .transform
                    .trans(item.x, item.y)
                    .trans(HALF_CELL_SIZE, HALF_CELL_SIZE);

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
        app_state,
    };

    let mut settings = EventSettings::new();
    settings.set_max_fps(30);
    let mut events = Events::new(settings);

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
                Button::Keyboard(Key::Space) => app.snake.growth += 1,
                Button::Keyboard(Key::Return) => app.snake.poop += 1,
                _ => {}
            }
        }
    }
}
