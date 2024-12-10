use eframe::egui;
use rand::Rng;

const BOARD_WIDTH: usize = 20;
const BOARD_HEIGHT: usize = 20;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

struct SnakeGame {
    snake: Vec<Point>,
    direction: Direction,
    food: Point,
    score: usize,
    game_over: bool,
}

impl SnakeGame {
    fn new() -> Self {
        let snake = vec![Point { x: 10, y: 10 }];
        let food = Point { x: 5, y: 5 };

        Self {
            snake,
            direction: Direction::Right,
            food,
            score: 0,
            game_over: false,
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        // Move the snake's head
        let mut new_head = *self.snake.first().unwrap();
        match self.direction {
            Direction::Up => {
                if new_head.y == 0 {
                    self.game_over = true;
                    return;
                }
                new_head.y -= 1;
            }
            Direction::Down => {
                if new_head.y == BOARD_HEIGHT - 1 {
                    self.game_over = true;
                    return;
                }
                new_head.y += 1;
            }
            Direction::Left => {
                if new_head.x == 0 {
                    self.game_over = true;
                    return;
                }
                new_head.x -= 1;
            }
            Direction::Right => {
                if new_head.x == BOARD_WIDTH - 1 {
                    self.game_over = true;
                    return;
                }
                new_head.x += 1;
            }
        }

        // Check collision with itself
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        // Check if the snake eats the food
        if new_head == self.food {
            self.snake.insert(0, new_head); // Add a new segment to the snake
            self.score += 1;
            self.generate_food();
        } else {
            self.snake.insert(0, new_head);
            self.snake.pop(); // Remove the last segment
        }
    }

    fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let new_food = Point {
                x: rng.gen_range(0..BOARD_WIDTH),
                y: rng.gen_range(0..BOARD_HEIGHT),
            };
            if !self.snake.contains(&new_food) {
                self.food = new_food;
                break;
            }
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        // Prevent the snake from reversing direction
        if (self.direction == Direction::Up && new_direction == Direction::Down)
            || (self.direction == Direction::Down && new_direction == Direction::Up)
            || (self.direction == Direction::Left && new_direction == Direction::Right)
            || (self.direction == Direction::Right && new_direction == Direction::Left)
        {
            return;
        }
        self.direction = new_direction;
    }
}

struct SnakeApp {
    game: SnakeGame,
}

impl Default for SnakeApp {
    fn default() -> Self {
        Self {
            game: SnakeGame::new(),
        }
    }
}

impl eframe::App for SnakeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Game logic
        if !self.game.game_over {
            self.game.update();
            // Adjust the delay to slow down the snake
            ctx.request_repaint_after(std::time::Duration::from_millis(10000));
        }

        // GUI drawing
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Snake Game");

            if self.game.game_over {
                ui.label(format!("Game Over! Final Score: {}", self.game.score));
                if ui.button("Restart").clicked() {
                    self.game = SnakeGame::new();
                }
                return;
            }

            // Display the board
            for y in 0..BOARD_HEIGHT {
                ui.horizontal(|ui| {
                    for x in 0..BOARD_WIDTH {
                        let point = Point { x, y };

                        if self.game.snake.contains(&point) {
                            ui.colored_label(egui::Color32::GREEN, "⬛");
                        } else if self.game.food == point {
                            ui.colored_label(egui::Color32::RED, "🍎");
                        } else {
                            ui.label("⬜");
                        }
                    }
                });
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label(format!("Score: {}", self.game.score));
            });

            // Control buttons
            ui.horizontal(|ui| {
                if ui.button("⬆️").clicked() {
                    self.game.change_direction(Direction::Up);
                }
            });
            ui.horizontal(|ui| {
                if ui.button("⬅️").clicked() {
                    self.game.change_direction(Direction::Left);
                }
                if ui.button("➡️").clicked() {
                    self.game.change_direction(Direction::Right);
                }
            });
            ui.horizontal(|ui| {
                if ui.button("⬇️").clicked() {
                    self.game.change_direction(Direction::Down);
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Snake Game",
        options,
        Box::new(|_cc| Box::new(SnakeApp::default())),
    )
}
