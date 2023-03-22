use ggez::{
    conf::WindowMode,
    conf::WindowSetup,
    event,
    glam::*,
    graphics::{self, Color, PxScale, Text, TextFragment},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};
use nalgebra::Vector2;

mod consts;
use consts::{
    EARTH_SPEED, HEIGHT, JUPITER_SPEED, KM_IN_PX, MARS_SPEED, MERCURY_SPEED, MOON_SPEED,
    NEPTUNE_SPEED, NINTH_SPEED, PLUTO_SPEED, SATURN_SPEED, URANUS_SPEED, VENUS_SPEED, WIDTH,
};
mod planet;
use planet::Planet;
mod utils;
use utils::update_planet_position;

// ENGINE

struct MainState {
    planets: Vec<Planet>,
    world_scale: f32,
    scene_position: [f32; 2],
    last_mouse_position: Option<[f32; 2]>,
    curr_mouse_position: Option<[f32; 2]>,
    sim_speed: i32,
    dt_multiplier: f64,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let planets: Vec<Planet> = vec![
            Planet {
                label: String::from("Sun"),
                position: Vector2::new(0.0, 0.0),
                velocity: Vector2::new(0.0, 0.0),
                acceleration: Vector2::new(0.0, 0.0),
                radius: 1.0,
                mass: 1.989e30,
                color: [0.85, 0.65, 0.13, 1.0],
            },
            Planet {
                label: String::from("Mercury"),
                position: Vector2::new(-49.553e6, 0.0),

                velocity: Vector2::new(0.0, MERCURY_SPEED),
                acceleration: Vector2::new(0.0, 0.0),
                radius: 0.0030581,
                mass: 3.285e23,
                color: [0.74, 0.72, 0.42, 1.0],
            },
            Planet {
                label: String::from("Venus"),
                position: Vector2::new(-108.209e6, 0.0),
                velocity: Vector2::new(0.0, VENUS_SPEED),
                acceleration: Vector2::new(0.0, 0.0),
                radius: 0.0086956,
                mass: 4.867e24,
                color: [0.72, 0.45, 0.2, 1.0],
            },
            Planet {
                label: String::from("Earth"),
                position: Vector2::new(-149.6e6, 0.0),
                velocity: Vector2::new(0.0, EARTH_SPEED),
                acceleration: Vector2::new(0.0, 0.0),
                radius: 0.0091743,
                mass: 5.972e24,
                color: [0.61, 0.73, 0.5, 1.0],
            },
            Planet {
                label: String::from("Moon"),
                position: Vector2::new(-149.9835e6, 0.0),
                velocity: Vector2::new(0.0, MOON_SPEED),
                acceleration: Vector2::new(0.0, 0.0),
                radius: 0.0025,
                mass: 7.34767309e22,
                color: [1.0, 1.0, 1.0, 1.0],
            },
            Planet {
                label: String::from("Mars"),
                position: Vector2::new(-227.9e6, 0.0),
                velocity: Vector2::new(0.0, MARS_SPEED),
                acceleration: Vector2::new(0.0, 0.0),
                radius: 0.00458715,
                mass: 6.39e23,
                color: [0.78, 0.25, 0.09, 1.0],
            },
            Planet {
                label: String::from("Jupiter"),
                position: Vector2::new(-778.6e6, 0.0),
                velocity: Vector2::new(0.0, JUPITER_SPEED),
                acceleration: Vector2::new(0.0, 0.0),
                radius: 0.1283381,
                mass: 1.89813e27,
                color: [0.78, 0.25, 0.09, 1.0],
            },
            Planet {
                label: String::from("Saturn"),
                position: Vector2::new(-1433.5e6, 0.0),
                velocity: Vector2::new(0.0, SATURN_SPEED),
                acceleration: Vector2::new(0.0, 0.0),
                radius: 0.82566,
                mass: 5.683e26,
                color: [0.72, 0.45, 0.2, 1.0],
            },
            Planet {
                label: String::from("Uranus"),
                position: Vector2::new(-2872.5e6, 0.0),
                velocity: Vector2::new(0.0, URANUS_SPEED),
                acceleration: Vector2::new(0.0, 0.0),
                radius: 0.366972,
                mass: 8.681e25,
                color: [0.69, 0.85, 0.9, 1.0],
            },
            Planet {
                label: String::from("Neptune"),
                position: Vector2::new(-4496.1e6, 0.0),
                velocity: Vector2::new(0.0, NEPTUNE_SPEED),
                acceleration: Vector2::new(0.0, 0.0),
                radius: 0.366972,
                mass: 1.0241e26,
                color: [0.01, 0.02, 0.95, 1.0],
            },
            Planet {
                label: String::from("Pluto :)"),
                position: Vector2::new(-5906.38e6, 0.0),
                velocity: Vector2::new(0.0, PLUTO_SPEED),
                acceleration: Vector2::new(0.0, 0.0),
                radius: 0.366972,
                mass: 1.309e22,
                color: [0.87, 0.96, 0.98, 1.0],
            },
            Planet {
                label: String::from("9th Planet"),
                position: Vector2::new(-37400.0e6, 0.0),
                velocity: Vector2::new(0.0, NINTH_SPEED),
                acceleration: Vector2::new(0.0, 0.0),
                radius: 0.02293575,
                mass: 29.86e24,
                color: [0.5, 0.5, 0.5, 1.0],
            },
        ];

        // let planets: Vec<Planet> = vec![
        //     Planet {
        //         label: String::from("Sun"),
        //         position: Vec2 { x: 0.0, y: 0.0 },
        //         velocity: Vec2 { x: 0.0, y: -5000.0 },
        //         acceleration: Vec2 { x: 0.0, y: 0.0 },
        //         radius: 0.05,
        //         mass: 34.0e32,
        //         color: [0.85, 0.65, 0.13, 1.0],
        //     },
        //     Planet {
        //         label: String::from("Earth"),
        //         position: Vec2 { x: -10.0, y: 0.0 },
        //         velocity: Vec2 { x: 0.0, y: 5000.0 },
        //         acceleration: Vec2 { x: 0.0, y: 0.0 },
        //         radius: 0.05,
        //         mass: 34.0e32,
        //         color: [0.61, 0.73, 0.5, 1.0],
        //     },
        //     Planet {
        //         label: String::from("Earth"),
        //         position: Vec2 { x: -4.0, y: 6.0 },
        //         velocity: Vec2 { x: 8000.0, y: 0.0 },
        //         acceleration: Vec2 { x: 0.0, y: 0.0 },
        //         radius: 0.05,
        //         mass: 34.0e32,
        //         color: [0.61, 0.73, 0.5, 1.0],
        //     },
        // ];

        Ok(MainState {
            planets,
            world_scale: 2.5,
            scene_position: [WIDTH / 2.0, HEIGHT / 2.0],
            last_mouse_position: None,
            curr_mouse_position: None,
            sim_speed: 20_000,
            dt_multiplier: 1.0,
        })
    }

    fn zoom(&mut self, zoom_delta: f32) {
        // Adjust the world scale based on the zoom delta.
        let prev_scale = self.world_scale;
        self.world_scale += zoom_delta * self.world_scale;
        let dx = self.scene_position[0] - self.curr_mouse_position.unwrap_or([0.0, 0.0])[0];
        let dy = self.scene_position[1] - self.curr_mouse_position.unwrap_or([0.0, 0.0])[1];
        self.scene_position[0] =
            self.curr_mouse_position.unwrap_or([0.0, 0.0])[0] + dx / prev_scale * self.world_scale;
        self.scene_position[1] =
            self.curr_mouse_position.unwrap_or([0.0, 0.0])[1] + dy / prev_scale * self.world_scale;
        // if self.world_scale < 0.1 {
        //     self.world_scale = 0.1;
        // }
    }

    fn update_scene_position(&mut self, mouse_position: [f32; 2]) {
        if let Some(last_position) = self.last_mouse_position {
            let dx = last_position[0] - mouse_position[0];
            let dy = last_position[1] - mouse_position[1];
            self.scene_position[0] -= dx;
            self.scene_position[1] -= dy;
        }
        self.last_mouse_position = Some(mouse_position);
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn mouse_wheel_event(&mut self, _ctx: &mut ggez::Context, _x: f32, y: f32) -> GameResult<()> {
        // Zoom in or out based on the scroll wheel input.
        let zoom_factor = 0.1;
        if y > 0.0 {
            self.zoom(zoom_factor);
        } else if y < 0.0 {
            self.zoom(-zoom_factor);
        }
        // handle the scroll event here
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> GameResult<()> {
        if let Some(_) = self.last_mouse_position {
            let mouse_position = [x, y];
            self.update_scene_position(mouse_position);
        }
        self.curr_mouse_position = Some([x, y]);
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult<()> {
        let mouse_position = [x, y];
        self.update_scene_position(mouse_position);

        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult<()> {
        self.last_mouse_position = None;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Equals) => {
                if self.sim_speed >= 30_000 {
                    self.dt_multiplier += 2.0;
                } else {
                    self.sim_speed += 1000;
                }
            }
            Some(KeyCode::Minus) => {
                if self.sim_speed >= 30_000 {
                    self.dt_multiplier -= 2.0;
                } else {
                    self.sim_speed -= 1000;
                    if self.sim_speed < 1 {
                        self.sim_speed = 1;
                    }
                }
            }
            Some(KeyCode::R) => {
                self.world_scale = 1.0;
                self.sim_speed = 20_000;
                self.scene_position = [0.0, 0.0];
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for _ in 0..self.sim_speed {
            for i in 0..self.planets.len() {
                update_planet_position(i, &mut self.planets, self.dt_multiplier);
                // check_collisions(i, &mut self.planets, self.sim_speed);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));

        for i in 0..self.planets.len() {
            let new_planet = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                vec2(0., 0.),
                (self.planets[i].radius) * self.world_scale,
                0.2,
                Color::from(self.planets[i].color),
            )?;
            let fragment = TextFragment::new(&self.planets[i].label).color(self.planets[i].color);
            let mut label = Text::new(fragment);

            label.set_scale(PxScale::from(24.0));

            canvas.draw(
                &new_planet,
                Vec2::new(
                    (self.planets[i].position.x / KM_IN_PX) as f32 * self.world_scale
                        + self.scene_position[0],
                    (self.planets[i].position.y / KM_IN_PX) as f32 * self.world_scale
                        + self.scene_position[1],
                ),
            );

            canvas.draw(
                &label,
                Vec2::new(
                    (self.planets[i].position.x / KM_IN_PX - 0.05) as f32 * self.world_scale
                        + self.scene_position[0],
                    (self.planets[i].position.y / KM_IN_PX - 0.05) as f32 * self.world_scale
                        + self.scene_position[1],
                ),
            );
        }

        let speed_ind_text = format!("Speed: {}x RL", self.sim_speed * self.dt_multiplier as i32);
        let mut speed_ind = Text::new(speed_ind_text);
        speed_ind.set_scale(PxScale::from(30.0));
        canvas.draw(&speed_ind, Vec2::new(20.0, 20.0));

        canvas.finish(ctx)?;

        Ok(())
    }
}

// MAIN FUNCTION

pub fn main() -> GameResult {
    let window_setup = WindowSetup::default().title("Gravity Simultion");
    let window_mode = WindowMode::default().dimensions(WIDTH, HEIGHT);
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
