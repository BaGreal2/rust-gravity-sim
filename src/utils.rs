use crate::consts::{DT, G, KM_IN_PX};
use crate::planet::Planet;
// use ggez::glam::Vec2;
use nalgebra::Vector2;
// use num_bigfloat::BigFloat;

pub fn update_planet_position(i: usize, planets: &mut Vec<Planet>, dt_multiplier: f64) {
    let mut neigbours_forces: Vec<Vector2<f64>> = vec![];

    for j in 0..planets.len() {
        if j == i {
            continue;
        };

        let diff = (planets[i].position - planets[j].position).norm() / KM_IN_PX;

        if diff as f32 <= planets[i].radius + planets[j].radius + 0.5
            || (diff >= 5.0 && planets[j].mass <= 1.0e30)
        {
            continue;
        }

        let curr_force = calculate_gravity_force(
            planets[i].mass,
            planets[i].position,
            planets[j].mass,
            planets[j].position,
        );

        neigbours_forces.push(curr_force);
    }

    let mut result_force: Vector2<f64> = Vector2::new(0.0, 0.0);

    for force in neigbours_forces {
        result_force += force;
    }

    let acceleration = result_force / planets[i].mass;

    planets[i].velocity += acceleration * DT * dt_multiplier;

    let vel = planets[i].velocity;

    planets[i].position += vel * DT * dt_multiplier;

    // if planets[i].label == "Mercury" {
    //     println!("Merc pos: {:?}", planets[i].position);
    // }
}

fn calculate_gravity_force(m1: f64, p1: Vector2<f64>, m2: f64, p2: Vector2<f64>) -> Vector2<f64> {
    let diff = p2 - p1;
    let r: f64 = diff.norm();

    let dir = diff.normalize();
    let magnitude = G * (m1 * m2) / (r * r);

    dir * magnitude
}

// pub fn check_collisions(i: usize, planets: &mut Vec<Planet>, sim_speed: i32) -> bool {
//     for j in 0..planets.len() {
//         if j == i {
//             continue;
//         }

//         let adj_dt = DT * sim_speed as f32;
//         let speed_i = planets[i].vx / 696000000.0 * adj_dt;
//         let speed_j = planets[j].vx / 696000000.0 * adj_dt;
//         if get_distance(planets[i].x, planets[i].y, planets[j].x, planets[j].y)
//             <= (planets[i].radius + planets[j].radius) + speed_i + speed_j
//         {
//             let neighbout_mass = planets[j].mass;
//             planets[i].mass += neighbout_mass;
//             planets[i].radius += planets[j].radius / 2.0;

//             planets.remove(j);

//             return true;
//         }
//     }
//     return false;
// }

// fn get_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
//     let dx = x2 - x1;
//     let dy = y2 - y1;
//     let dist = f32::sqrt(dx * dx + dy * dy);
//     dist as f32
// }
