extern crate pancurses;

use pancurses::{initscr, endwin, noecho, Window, Input};
use std::collections::VecDeque;
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

struct Snake {
    body: VecDeque<Coordinate>,
    direction: Direction,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Food {
    position: Coordinate,
}

fn main() {
    // Initialize the window
    let window = initscr();
    window.keypad(true);
    window.timeout(1); // Non-blocking getch
    noecho();

    // Initialize game objects
    let mut snake = Snake {
        body: VecDeque::from(vec![Coordinate { x: 10, y: 10 }]),
        direction: Direction::Right,
    };

    let mut food = create_food(&window);

    // Main game loop
    loop {
        // Handle user input
        match window.getch() {
            Some(Input::Character('q')) => break,
            Some(Input::KeyUp) => snake.direction = Direction::Up,
            Some(Input::KeyDown) => snake.direction = Direction::Down,
            Some(Input::KeyLeft) => snake.direction = Direction::Left,
            Some(Input::KeyRight) => snake.direction = Direction::Right,
            _ => {}
        }

        // Update game state
        if let Err(_) = update_snake(&mut snake, &window) {
            // Snake has collided with the wall or itself
            render_game_over(&window);
            break;
        }

        if check_food_collision(&snake, &food) {
            grow_snake(&mut snake);
            food = create_food(&window);
        }

        // Render game
        render(&window, &snake, &food);
    }

    // Clean up
    endwin();
}

fn update_snake(snake: &mut Snake, window: &Window) -> Result<(), ()> {
    let mut new_head = *snake.body.front().expect("Snake has no body");
    match snake.direction {
        Direction::Up => new_head.y -= 1,
        Direction::Down => new_head.y += 1,
        Direction::Left => new_head.x -= 1,
        Direction::Right => new_head.x += 1,
    }

    // Check for collisions with walls
    if new_head.x < 0 || new_head.x >= window.get_max_x() || new_head.y < 0 || new_head.y >= window.get_max_y() {
        return Err(());
    }

    // Check for collisions with itself
    if snake.body.contains(&new_head) {
        return Err(());
    }

    snake.body.push_front(new_head);
    snake.body.pop_back();
    Ok(())
}

fn check_food_collision(snake: &Snake, food: &Food) -> bool {
    snake.body.front() == Some(&food.position)
}

fn grow_snake(snake: &mut Snake) {
    let tail = *snake.body.back().expect("Snake has no body");
    snake.body.push_back(tail);
}

fn create_food(window: &Window) -> Food {
    let mut rng = rand::thread_rng();
    let max_x = window.get_max_x();
    let max_y = window.get_max_y();
    Food {
        position: Coordinate {
            x: rng.gen_range(1..max_x - 1),
            y: rng.gen_range(1..max_y - 1),
        },
    }
}

fn render(window: &Window, snake: &Snake, food: &Food) {
    window.clear();
    for point in &snake.body {
        window.mvaddch(point.y, point.x, '@');
    }
    window.mvaddch(food.position.y, food.position.x, 'O');
    window.refresh();
}

fn render_game_over(window: &Window) {
    window.clear();
    let (max_y, max_x) = window.get_max_yx();
    window.mvaddstr(max_y / 2, (max_x / 2) - 5, "Game Over!");
    window.refresh();
    window.getch();
}
