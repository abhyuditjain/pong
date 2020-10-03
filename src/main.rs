use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::input::keyboard;
use ggez::mint::Vector2;
use ggez::{Context, GameResult};

type Vector = Vector2<f32>;

const SCREEN_HEIGHT: f32 = 600.;
const SCREEN_WIDTH: f32 = 1200.;

const X_OFFSET: f32 = 20.;
// distance from each paddle to their respective walls
const PADDLE_WIDTH: f32 = 12.;
const PADDLE_HEIGHT: f32 = 75.;
const PADDLE_SPEED: f32 = 5.0;

const BALL_RADIUS: f32 = 10.;
const BALL_MIN_VEL: f32 = 2.0;
const BALL_MAX_VEL: f32 = 3.0;

struct Ball {
    ball: Rect,
    vel: Vector,
}

impl Ball {
    fn new() -> Self {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng(); // initialize random number generator
        let mut x_vel = rng.gen_range(BALL_MIN_VEL, BALL_MAX_VEL); // generate random float from 3 to 5
        let mut y_vel = rng.gen_range(BALL_MIN_VEL, BALL_MAX_VEL);

        // rng.gen::<bool> generates either true or false with a 50% chance of each
        // Used for initial direction of ball
        if rng.gen::<bool>() {
            x_vel *= -1.0;
        }
        if rng.gen::<bool>() {
            y_vel *= -1.0;
        }

        Self {
            ball: Rect::new(
                SCREEN_WIDTH / 2.0 - BALL_RADIUS / 2.0,
                SCREEN_HEIGHT / 2.0 - BALL_RADIUS / 2.0,
                BALL_RADIUS,
                BALL_RADIUS,
            ),
            vel: Vector { x: x_vel, y: y_vel },
        }
    }
}

struct MainState {
    l_paddle: Rect,
    r_paddle: Rect,
    ball: Ball,
    l_score: u16,
    r_score: u16,
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) && self.l_paddle.top() > 0.0 {
            self.l_paddle.y -= PADDLE_SPEED;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::S)
            && self.l_paddle.bottom() < SCREEN_HEIGHT
        {
            self.l_paddle.y += PADDLE_SPEED;
        }

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Up) && self.r_paddle.top() > 0.0 {
            self.r_paddle.y -= PADDLE_SPEED;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Down)
            && self.r_paddle.bottom() < SCREEN_HEIGHT
        {
            self.r_paddle.y += PADDLE_SPEED;
        }

        if (self.ball.vel.x < 0.0 && self.ball.ball.overlaps(&self.l_paddle))
            || (self.ball.vel.x > 0.0 && self.ball.ball.overlaps(&self.r_paddle))
        {
            self.ball.vel.x *= -1.0;
        }

        if (self.ball.vel.y < 0.0 && self.ball.ball.top() < 0.0)
            || (self.ball.vel.y > 0.0 && self.ball.ball.bottom() > SCREEN_HEIGHT)
        {
            self.ball.vel.y *= -1.0;
        }

        if self.ball.ball.left() < 0.0 {
            self.r_score += 1;
            std::thread::sleep(std::time::Duration::from_millis(1000));
            self.ball = Ball::new();
        }

        if self.ball.ball.right() > SCREEN_WIDTH {
            self.l_score += 1;
            std::thread::sleep(std::time::Duration::from_millis(1000));
            self.ball = Ball::new();
        }

        self.ball.ball.translate(self.ball.vel);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use ggez::graphics::{clear, present, Color};

        clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        // All drawing here and then present

        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.ball.ball,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .expect("error creating ball mesh");

        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())
            .expect("error drawing ball mesh");

        let l_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.l_paddle,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .expect("error creating ball mesh");
        graphics::draw(ctx, &l_paddle_mesh, graphics::DrawParam::default())
            .expect("error drawing ball mesh");

        let r_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.r_paddle,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .expect("error creating ball mesh");
        graphics::draw(ctx, &r_paddle_mesh, graphics::DrawParam::default())
            .expect("error drawing ball mesh");

        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())
            .expect("error drawing ball mesh");

        let mut scoreboard_text =
            graphics::Text::new(format!("L: {} \t R: {}", self.l_score, self.r_score));
        scoreboard_text.set_font(graphics::Font::default(), graphics::Scale::uniform(24.0));

        let coords = [
            SCREEN_WIDTH / 2.0 - scoreboard_text.width(ctx) as f32 / 2.0,
            10.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        graphics::draw(ctx, &scoreboard_text, params).expect("error drawing scoreboard text");

        present(ctx).expect("Error presenting");

        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Pong", "Abhyudit Jain")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();

    let main_state = &mut MainState {
        l_paddle: Rect::new(
            X_OFFSET,
            SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        ),
        r_paddle: Rect::new(
            SCREEN_WIDTH - X_OFFSET,
            SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        ),
        ball: Ball::new(),
        l_score: 0,
        r_score: 0,
    };

    ggez::event::run(ctx, event_loop, main_state)
}
