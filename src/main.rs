use ::rand::{self, Rng};
use macroquad::prelude::*;

const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_SPEED: f32 = 8.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_SPEED: f32 = 10.0;
const OFFSET: f32 = 50.0;

struct Player {
    score: u32,
    is_ai: bool,
    x: f32,
    y: f32,
}

impl Player {
    fn new() -> Self {
        let score = 0;
        let is_ai = false;
        let x = OFFSET;
        let y = screen_height() / 2.0;

        Self { score, is_ai, x, y }
    }

    fn new_ai() -> Self {
        let score = 0;
        let is_ai = true;
        let x = screen_width() - OFFSET;
        let y = screen_height() / 2.0;

        Self { score, is_ai, x, y }
    }

    fn draw(&self) {
        if self.is_ai {
            self.draw_ai();
        } else {
            self.draw_player();
        }
    }

    fn draw_ai(&self) {
        let ai_score = &self.score.to_string();
        let ai_text = measure_text(ai_score, None, OFFSET as u16, 1.0);

        draw_text(
            ai_score,
            screen_width() / 2.0 - ai_text.width / 2.0,
            screen_height() - ai_text.height / 2.0,
            OFFSET,
            WHITE,
        );
        draw_rectangle(
            self.x,
            self.y - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            WHITE,
        );
    }

    fn draw_player(&self) {
        let player_score = &self.score.to_string();
        let player_text = measure_text(player_score, None, OFFSET as u16, 1.0);

        draw_text(
            player_score,
            screen_width() / 2.0 - player_text.width / 2.0,
            OFFSET - player_text.height / 2.0,
            OFFSET,
            WHITE,
        );
        draw_rectangle(
            self.x,
            self.y - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            WHITE,
        );
    }

    fn ai_mv(&mut self, ball: &Ball) {
        let can_mv_up = self.y - PADDLE_SPEED - PADDLE_HEIGHT / 2.0 >= 0.0;
        let can_mv_down = self.y + PADDLE_SPEED + PADDLE_HEIGHT / 2.0 <= screen_height();
        if self.x < screen_width() / 2.0 {
            if ball.x <= screen_width() / 2.0 {
                if ball.y > self.y && can_mv_down {
                    self.y += PADDLE_SPEED;
                } else if ball.y < self.y && can_mv_up {
                    self.y -= PADDLE_SPEED;
                }
            }
        } else {
            if ball.x >= screen_width() / 2.0 {
                if ball.y > self.y && can_mv_down {
                    self.y += PADDLE_SPEED;
                } else if ball.y < self.y && can_mv_up {
                    self.y -= PADDLE_SPEED;
                }
            }
        }
    }

    fn player_mv(&mut self) {
        let can_mv_up = self.y - PADDLE_SPEED - PADDLE_HEIGHT / 2.0 >= 0.0;
        let can_mv_down = self.y + PADDLE_SPEED + PADDLE_HEIGHT / 2.0 <= screen_height();

        if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)) && can_mv_up {
            self.y -= PADDLE_SPEED;
        }

        if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)) && can_mv_down {
            self.y += PADDLE_SPEED;
        }
    }
}

struct Ball {
    x: f32,
    y: f32,
    rot: f32,
    rng: rand::rngs::ThreadRng,
}

impl Ball {
    fn new() -> Self {
        let x = screen_width() / 2.0;
        let y = screen_height() / 2.0;
        let mut rng = rand::thread_rng();
        let rot = rng.gen_range(0.0..=360.0);

        Self { x, y, rot, rng }
    }

    fn rest(&mut self) {
        self.x = screen_width() / 2.0;
        self.y = screen_height() / 2.0;
        self.rot = self.rng.gen_range(0.0..=360.0);
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, BALL_RADIUS, WHITE);
    }

    fn mv(&mut self) {
        self.x += BALL_SPEED * self.rot.to_radians().cos();
        self.y += BALL_SPEED * self.rot.to_radians().sin();
    }
}

struct Game {
    person: Player,
    ai: Player,
    ball: Ball,
    is_running: bool,
}

impl Game {
    fn new() -> Self {
        let person = Player::new();
        let ai = Player::new_ai();
        let ball = Ball::new();
        let is_running = true;

        Self {
            person,
            ai,
            ball,
            is_running,
        }
    }

    fn draw(&self) {
        clear_background(BLACK);
        self.person.draw();
        self.ai.draw();
        self.ball.draw();

        for i in (OFFSET as u32..=(screen_height() - OFFSET) as u32).step_by(10) {
            draw_line(
                screen_width() / 2.0,
                i as f32,
                screen_width() / 2.0,
                i as f32 + 1.0,
                1.0,
                WHITE,
            );
        }
    }

    fn mv(&mut self) {
        self.person.player_mv();
        //self.person.ai_mv(&self.ball);
        self.ai.ai_mv(&self.ball);
        self.ball.mv();
        self.check_collision();
    }

    fn check_collision(&mut self) {
        // top of the screen
        if self.ball.y - BALL_RADIUS <= 0.0 {
            self.ball.rot = -self.ball.rot;
        }
        // bottom of the screen
        if self.ball.y + BALL_RADIUS >= screen_height() {
            self.ball.rot = -self.ball.rot;
        }
        // left of the screen
        if self.ball.x - BALL_RADIUS <= 0.0 {
            self.ai.score += 1;
            self.ball.rest();
        }
        // right of the screen
        if self.ball.x + BALL_RADIUS >= screen_width() {
            self.person.score += 1;
            self.ball.rest();
        }
        // left paddle
        if self.ball.x - BALL_RADIUS <= self.person.x + PADDLE_WIDTH / 2.0
            && self.ball.x - BALL_RADIUS >= self.person.x - PADDLE_WIDTH / 2.0
            && self.ball.y - BALL_RADIUS <= self.person.y + PADDLE_HEIGHT / 2.0
            && self.ball.y + BALL_RADIUS >= self.person.y - PADDLE_HEIGHT / 2.0
        {
            self.ball.rot += 180.0;
        }
        // right paddle
        if self.ball.x + BALL_RADIUS >= self.ai.x - PADDLE_WIDTH / 2.0
            && self.ball.x + BALL_RADIUS <= self.ai.x + PADDLE_WIDTH / 2.0
            && self.ball.y - BALL_RADIUS <= self.ai.y + PADDLE_HEIGHT / 2.0
            && self.ball.y + BALL_RADIUS >= self.ai.y - PADDLE_HEIGHT / 2.0
        {
            self.ball.rot -= 180.0;
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    while game.is_running {
        game.draw();
        game.mv();
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_string(),
        window_resizable: false,
        ..Default::default()
    }
}
