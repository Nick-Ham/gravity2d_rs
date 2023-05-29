use glam::Vec2;
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;

const FRAMES_PER_SECOND: u32 = 120;
const TIMESTEP: f32 = 1.0 / FRAMES_PER_SECOND as f32;

const G: f32 = 1.0;

const SHOULD_DRAW_VELOCITY: bool = false;
const DRAW_VELOCITY_LENGTH: f32 = 30.0;
const DRAW_VELOCITY_COLOR: Color = Color::BLUE;
const PLANET_COLOR: Color = Color::WHITE;

macro_rules! get_gravitational_acceleration_to_body {
    ($attracting_mass:expr, $radius:expr) => {
        (G * $attracting_mass) / ($radius)
    };
}

pub struct MassComponent {
    position: Vec2,
    mass: f32,
    radius: f32,
    velocity: Vec2,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("gravity2d_rs")
        .build();

    let mut mass_components: Vec<MassComponent> = Vec::new();
    make_mass_components(&mut mass_components);

    let mut time_since_last_update = 0.0;
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        time_since_last_update += dt;
        while time_since_last_update > TIMESTEP {
            time_since_last_update -= TIMESTEP;
            step_components(&mut mass_components);
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for mass_component in mass_components.iter() {
            draw_mass_component(mass_component, &mut d);
        }
    }
}

fn step_components(mass_components: &mut Vec<MassComponent>) {
    let mut mass_components_velocity_delta: Vec<Vec2> = Vec::new();
    let mut mass_components_position_delta: Vec<Vec2> = Vec::new();
    for i in 0..mass_components.len() {
        let mass_component = &mass_components[i];
        let mut net_acceleration = Vec2::new(0.0, 0.0);

        for j in 0..mass_components.len() {
            if i == j {
                continue;
            }

            let other_mass_component = &mass_components[j];

            let vector_to = other_mass_component.position - mass_component.position;
            let radius = vector_to.length();
            let direction_to = vector_to.normalize();
            let gravitational_acceleration = direction_to
                * get_gravitational_acceleration_to_body!(other_mass_component.mass, radius);
            //direction_to * (G * other_mass_component.mass) / radius;
            net_acceleration += gravitational_acceleration;
        }

        let velocity_delta =
            net_acceleration / (FRAMES_PER_SECOND as f32 * FRAMES_PER_SECOND as f32);
        mass_components_velocity_delta.push(velocity_delta);

        let position_delta = mass_component.velocity / FRAMES_PER_SECOND as f32;
        mass_components_position_delta.push(position_delta);
    }

    for i in 0..mass_components.len() {
        let mass_component = &mut mass_components[i];
        mass_component.velocity += mass_components_velocity_delta[i];
        mass_component.position += mass_components_position_delta[i];
    }
}

fn draw_mass_component(component: &MassComponent, draw_handle: &mut RaylibDrawHandle) {
    draw_handle.draw_circle(
        component.position.x as i32,
        component.position.y as i32,
        component.radius,
        PLANET_COLOR,
    );
    if SHOULD_DRAW_VELOCITY {
        let velocity_target =
            component.position + component.velocity.normalize() * DRAW_VELOCITY_LENGTH;
        draw_handle.draw_line(
            component.position.x as i32,
            component.position.y as i32,
            velocity_target.x as i32,
            velocity_target.y as i32,
            DRAW_VELOCITY_COLOR,
        );
    }
}

fn make_mass_component(position: Vec2, mass: f32, radius: f32, velocity: Vec2) -> MassComponent {
    let new_gravity: MassComponent = MassComponent {
        position,
        mass,
        radius,
        velocity,
    };
    new_gravity
}

fn make_mass_components(mass_components: &mut Vec<MassComponent>) {
    // Make large planet
    let new_position = Vec2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0);
    let new_mass = 1000000.0;
    let new_radius = 25.0;
    let initial_velocity = Vec2::new(-7.0, 1.0);
    let mass_component = make_mass_component(new_position, new_mass, new_radius, initial_velocity);
    mass_components.push(mass_component);

    // Make medium planet
    let new_position = Vec2::new(
        (WINDOW_WIDTH as f32 / 2.0) + 0.0,
        (WINDOW_HEIGHT as f32 / 2.0) + -100.0,
    );
    let new_mass = 5000.0;
    let new_radius = 16.0;
    let initial_velocity = Vec2::new(145.0, 0.0);
    let mass_component = make_mass_component(new_position, new_mass, new_radius, initial_velocity);
    mass_components.push(mass_component);

    // Make small planet
    let new_position = Vec2::new(
        (WINDOW_WIDTH as f32 / 2.0) + 100.0,
        (WINDOW_HEIGHT as f32 / 2.0) + 0.0,
    );
    let new_mass = 50.0;
    let new_radius = 4.0;
    let initial_velocity = Vec2::new(0.0, 50.0);
    let mass_component = make_mass_component(new_position, new_mass, new_radius, initial_velocity);
    mass_components.push(mass_component);

    let new_position = Vec2::new(
        (WINDOW_WIDTH as f32 / 2.0) + 0.0,
        (WINDOW_HEIGHT as f32 / 2.0) + 100.0,
    );
    let new_mass = 30.0;
    let new_radius = 3.0;
    let initial_velocity = Vec2::new(-50.0, 0.0);
    let mass_component = make_mass_component(new_position, new_mass, new_radius, initial_velocity);
    mass_components.push(mass_component);

    let new_position = Vec2::new(
        (WINDOW_WIDTH as f32 / 2.0) + -100.0,
        (WINDOW_HEIGHT as f32 / 2.0) + 0.0,
    );
    let new_mass = 50.0;
    let new_radius = 4.0;
    let initial_velocity = Vec2::new(0.0, -50.0);
    let mass_component = make_mass_component(new_position, new_mass, new_radius, initial_velocity);
    mass_components.push(mass_component);

    // Make small planet
    let new_position = Vec2::new(
        (WINDOW_WIDTH as f32 / 2.0) + 75.0,
        (WINDOW_HEIGHT as f32 / 2.0) + 0.0,
    );
    let new_mass = 20.0;
    let new_radius = 2.0;
    let initial_velocity = Vec2::new(0.0, 75.0);
    let mass_component = make_mass_component(new_position, new_mass, new_radius, initial_velocity);
    mass_components.push(mass_component);

    let new_position = Vec2::new(
        (WINDOW_WIDTH as f32 / 2.0) + 0.0,
        (WINDOW_HEIGHT as f32 / 2.0) + 75.0,
    );
    let new_mass = 20.0;
    let new_radius = 2.0;
    let initial_velocity = Vec2::new(-75.0, 0.0);
    let mass_component = make_mass_component(new_position, new_mass, new_radius, initial_velocity);
    mass_components.push(mass_component);

    let new_position = Vec2::new(
        (WINDOW_WIDTH as f32 / 2.0) + -75.0,
        (WINDOW_HEIGHT as f32 / 2.0) + 0.0,
    );
    let new_mass = 20.0;
    let new_radius = 2.0;
    let initial_velocity = Vec2::new(0.0, -75.0);
    let mass_component = make_mass_component(new_position, new_mass, new_radius, initial_velocity);
    mass_components.push(mass_component);
}
