extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::*;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::EventLoop;
use rand::prelude::*;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::LinkedList;
use std::rc::Rc;

const WINDOW_WIDTH: f64 = 400.0;
const WINDOW_HEIGHT: f64 = 400.0;
const HALF_CELL_SIZE: f64 = 5.0;

const STEP_MULTIPLICATOR: f64 = 100.0;
const GROW_MULTIPLICATOR: i64 = 4;
const STEPS_WITHOUT_ROTATION: i64 = 15;
const INITIAL_SIZE: usize = 12;
const INITIAL_HP: i64 = 3;

const POISON_DROP_CHANCE: i64 = 20; //%
const POISON_DROP_CHANCE_RANGE_NEXT: i64 = POISON_DROP_CHANCE + 1;
const HEAL_DROP_CHANCE: i64 = 7; // %
const HEAL_DROP_CHANCE_RANGE_TO: i64 = POISON_DROP_CHANCE_RANGE_NEXT + HEAL_DROP_CHANCE;
// const HEAL_DROP_CHANCE_RANGE_NEXT: i64 = HEAL_DROP_CHANCE + 1;

const POOPING_CHANCE: i64 = 8; // 0.8%
const INVULNERABILITY_THRESHOLD: usize = 50;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BROWN: [f32; 4] = [0.76, 0.33, 0.08, 1.0];

#[derive(Copy, Clone, Debug)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_invert(&self, other: &Direction) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ChainType {
    Snake,
    Poop,
    Heal,
    Poison,
}

impl ChainType {
    fn get_color(&self) -> [f32; 4] {
        match self {
            ChainType::Poop => BROWN,
            ChainType::Snake => BLUE,
            ChainType::Poison => GREEN,
            ChainType::Heal => RED,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct ChainLink {
    x: f64,
    y: f64,
    t: ChainType,
    id: u64, // Add unique ID for safe comparison
}

impl Eq for ChainLink {}
impl Ord for ChainLink {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // in this business logic we cannot get NaN f64 values
        self.partial_cmp(other).unwrap()
    }
}

impl ChainLink {
    fn new(x: f64, y: f64, t: ChainType, id: u64) -> ChainLink {
        ChainLink { x, y, t, id }
    }

    // Fixed rectangle intersection algorithm
    fn intersects(&self, other: &ChainLink) -> bool {
        let this_x1 = self.x - HALF_CELL_SIZE;
        let this_x2 = self.x + HALF_CELL_SIZE;
        let this_y1 = self.y - HALF_CELL_SIZE;
        let this_y2 = self.y + HALF_CELL_SIZE;
        let other_x1 = other.x - HALF_CELL_SIZE;
        let other_x2 = other.x + HALF_CELL_SIZE;
        let other_y1 = other.y - HALF_CELL_SIZE;
        let other_y2 = other.y + HALF_CELL_SIZE;

        // Proper rectangle intersection: check if rectangles DON'T overlap, then negate
        !(this_x2 < other_x1 || other_x2 < this_x1 || this_y2 < other_y1 || other_y2 < this_y1)
    }
}

#[derive(Clone)]
struct Snake {
    direction: Direction,
    chain: LinkedList<ChainLink>,
    growth: i64,
    poop: i64,
    app_state: Rc<RefCell<AppState>>,
    hp: i64,
    cannot_rotate_steps: i64,
    deffered_rotation: Direction,
    rnd: ThreadRng,
    score: i64,
    next_id: u64, // Counter for generating unique IDs
}

impl Snake {
    fn new(app_state: Rc<RefCell<AppState>>) -> Snake {
        let mut snake = Snake {
            chain: LinkedList::new(),
            direction: Direction::None,
            growth: 0,
            poop: 0,
            app_state,
            hp: INITIAL_HP,
            rnd: rand::thread_rng(),
            cannot_rotate_steps: 0,
            deffered_rotation: Direction::None,
            score: 0,
            next_id: 0,
        };

        // Initialize snake chain with unique IDs
        for _ in 0..INITIAL_SIZE {
            let snake_chain = ChainLink::new(0.0, 0.0, ChainType::Snake, snake.next_id);
            snake.next_id += 1;
            snake.chain.push_back(snake_chain);
        }

        snake
    }

    fn new_direction(&mut self, direction: Direction) {
        if !self.direction.is_invert(&direction) {
            if self.cannot_rotate_steps <= 0 {
                self.direction = direction;
                self.cannot_rotate_steps = STEPS_WITHOUT_ROTATION;
            } else {
                self.deffered_rotation = direction;
            }
        }
    }

    fn make_deffered_rotation(&mut self) {
        if !matches!(self.deffered_rotation, Direction::None) && self.cannot_rotate_steps <= 0 {
            if !self.direction.is_invert(&self.deffered_rotation) {
                self.direction = self.deffered_rotation;
            }
            self.deffered_rotation = Direction::None;
        }
    }

    fn get_next_coords(&self, step_size: f64) -> Option<(f64, f64)> {
        let old_head = self.chain.front()?; // Safe access with Option

        let mut new_y = old_head.y;
        let mut new_x = old_head.x;

        match self.direction {
            Direction::Up => new_y -= step_size * STEP_MULTIPLICATOR,
            Direction::Down => new_y += step_size * STEP_MULTIPLICATOR,
            Direction::Left => new_x -= step_size * STEP_MULTIPLICATOR,
            Direction::Right => new_x += step_size * STEP_MULTIPLICATOR,
            _ => return None,
        };

        // teleport if snake trying to leave out of window
        if new_x > WINDOW_WIDTH - HALF_CELL_SIZE {
            new_x -= WINDOW_WIDTH;
        } else if new_x < HALF_CELL_SIZE {
            new_x += WINDOW_WIDTH;
        } else if new_y > WINDOW_HEIGHT - HALF_CELL_SIZE {
            new_y -= WINDOW_HEIGHT;
        } else if new_y < HALF_CELL_SIZE {
            new_y += WINDOW_HEIGHT;
        }
        Some((new_x, new_y))
    }

    fn make_step(&mut self, step_size: f64) {
        if self.is_dead() || matches!(self.direction, Direction::None) {
            return;
        }
        self.cannot_rotate_steps -= 1;
        self.make_deffered_rotation();

        // Safe coordinate calculation
        let (new_x, new_y) = match self.get_next_coords(step_size) {
            Some(coords) => coords,
            None => return, // Skip step if no valid coordinates
        };

        let new_head = ChainLink::new(new_x, new_y, ChainType::Snake, self.next_id);
        self.next_id += 1;
        self.chain.push_front(new_head);

        self.find_something_to_eat();
        self.check_collision_with_tail();
        self.shit_generation();

        // dropping tail business logic
        if self.growth == 0 && self.poop == 0 {
            self.chain.pop_back();
        } else if self.growth > 0 {
            self.growth -= 1;
        } else if self.poop > 0 {
            self.poop -= 1;

            if let Some(mut tail) = self.chain.pop_back() {
                // Fixed: use consistent random range generation (0..100)
                let chain_type = match self.rnd.gen_range(0..100) {
                    0..=POISON_DROP_CHANCE => ChainType::Poison,
                    POISON_DROP_CHANCE_RANGE_NEXT..=HEAL_DROP_CHANCE_RANGE_TO => ChainType::Heal,
                    _ => ChainType::Poop,
                };

                let mut app_state = self.app_state.borrow_mut();
                tail.t = chain_type;
                app_state.add_item_to_map(tail);
            }
        }
    }

    fn find_something_to_eat(&mut self) {
        let mut intersection: Option<ChainLink> = None;
        
        // Safe head access
        let head = match self.chain.front() {
            Some(h) => h,
            None => return,
        };

        {
            let app_state = self.app_state.borrow();
            let item_option = app_state
                .map_objects
                .iter()
                .find(|item| head.intersects(item));

            if let Some(item) = item_option {
                intersection = Some(*item);
            }
        }
        if let Some(item) = intersection {
            match item.t {
                ChainType::Poop => self.growth += GROW_MULTIPLICATOR,
                ChainType::Poison => self.hp -= 1,
                ChainType::Heal => self.hp += 1,
                _ => {}
            }
            self.score += 1;
            let mut app_state = self.app_state.borrow_mut();
            app_state.map_objects.remove(&item);
        }
    }

    // Fixed collision detection using unique IDs instead of pointer comparison
    fn check_collision_with_tail(&mut self) {
        let head = match self.chain.front() {
            Some(h) => h,
            None => return,
        };

        let collision_detected = self
            .chain
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > INVULNERABILITY_THRESHOLD)
            .any(|(_, item)| item.id != head.id && head.intersects(item));

        if collision_detected {
            self.hp = 0;
        }
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    fn shit_generation(&mut self) {
        // Fixed: use consistent random range generation (0..1000) and adjust comparison
        let random: i64 = self.rnd.gen_range(0..1000);

        if random <= POOPING_CHANCE {
            self.poop += 1;
        }
    }
}

#[derive(Clone)]
struct AppState {
    map_objects: BTreeSet<ChainLink>,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            map_objects: BTreeSet::new(),
        }
    }

    fn add_item_to_map(&mut self, item: ChainLink) {
        self.map_objects.insert(item);
    }
}

struct App<'a> {
    gl: GlGraphics,
    snake: Snake,
    app_state: Rc<RefCell<AppState>>,
    font: GlyphCache<'a>,
}

impl App<'_> {
    fn new(gl: GlGraphics, snake: Snake, app_state: Rc<RefCell<AppState>>) -> Result<App<'static>, Box<dyn std::error::Error>> {
        // Try multiple possible font paths for better compatibility
        let font_paths = [
            "./assets/PressStart2P-Regular.ttf",
            "assets/PressStart2P-Regular.ttf",
            "PressStart2P-Regular.ttf",
        ];

        let mut font_result = None;
        for path in &font_paths {
            if let Ok(font) = opengl_graphics::GlyphCache::new(path, (), TextureSettings::new()) {
                font_result = Some(font);
                break;
            }
        }

        let font = font_result.ok_or("Could not load font file. Please ensure PressStart2P-Regular.ttf is in the assets directory.")?;

        Ok(App {
            gl,
            snake,
            app_state,
            font,
        })
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let rect: [f64; 4] = rectangle::square(0.0, 0.0, 10.0);

        let snake = &self.snake;
        let app_state = &self.app_state;
        let font = &mut self.font;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            for snake_item in snake.chain.iter() {
                let transform = c
                    .transform
                    .trans(snake_item.x, snake_item.y)
                    .trans(HALF_CELL_SIZE, HALF_CELL_SIZE);

                rectangle(snake_item.t.get_color(), rect, transform, gl);
            }

            for item in app_state.borrow().map_objects.iter() {
                let transform = c
                    .transform
                    .trans(item.x, item.y)
                    .trans(HALF_CELL_SIZE, HALF_CELL_SIZE);
                rectangle(item.t.get_color(), rect, transform, gl);
            }

            let header = format!("COPRO SNAKE {} HP", snake.hp);
            let _ = text(BLACK, 15, &header, font, c.transform.trans(30.0, 30.0), gl);

            let score_header = format!("SCORE: {}", snake.score);
            let _ = text(
                BLACK,
                15,
                &score_header,
                font,
                c.transform.trans(20.0, 380.0),
                gl,
            );

            if snake.is_dead() {
                let _ = text(
                    BLACK,
                    30,
                    "GAME OVER",
                    font,
                    c.transform.trans(30.0, 70.0),
                    gl,
                );
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.snake.make_step(args.dt);
    }

    fn btn_press(&mut self, button: &Button) {
        match button {
            Button::Keyboard(Key::Up) => self.snake.new_direction(Direction::Up),
            Button::Keyboard(Key::Down) => self.snake.new_direction(Direction::Down),
            Button::Keyboard(Key::Left) => self.snake.new_direction(Direction::Left),
            Button::Keyboard(Key::Right) => self.snake.new_direction(Direction::Right),
            Button::Keyboard(Key::Space) => self.snake = Snake::new(self.app_state.clone()),
            Button::Keyboard(Key::Return) => self.restart(),
            _ => {}
        }
    }

    fn restart(&mut self) {
        let mut app_state = self.app_state.borrow_mut();
        app_state.map_objects = BTreeSet::new();
        self.snake = Snake::new(self.app_state.clone());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("snake", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()?;

    let app_state = Rc::new(RefCell::new(AppState::new()));
    let snake = Snake::new(app_state.clone());
    let mut app = App::new(GlGraphics::new(opengl), snake, app_state)?;

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
            app.btn_press(&button);
        }
    }

    Ok(())
}
