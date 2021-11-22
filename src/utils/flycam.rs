use bevy::input::mouse::MouseMotion;
use bevy::math::Vec3;
use bevy::prelude::*;

pub struct FlyCamPlugin;

impl Plugin for FlyCamPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(mouse_movement.system());
        app.add_system(camera_movement.system());
    }
}

pub struct FlyCam {
    pub top_speed: f32,
    pub friction: f32,
    pub acceleration: f32,
    pub velocity: Vec3,
    pub sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for FlyCam {
    fn default() -> Self {
        Self {
            acceleration: 1.25,
            friction: 1.0,
            top_speed: 0.25,
            sensitivity: 0.2,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

fn mouse_movement(
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut query: Query<(&mut FlyCam, &mut Transform)>,
) {
    let mut delta: Vec2 = Vec2::ZERO;

    for event in mouse_motion_event_reader.iter() {
        delta += event.delta;
    }
    if delta.is_nan() {
        return;
    }

    for (mut params, mut transform) in query.iter_mut() {
        // Apply the yaw.
        params.yaw -= delta.x * params.sensitivity;
        // params.yaw -= delta.x * params.sensitivity * time.delta_seconds();

        // Apply the pitch while clamping it.
        params.pitch += delta.y * params.sensitivity;
        // params.pitch += delta.y * params.sensitivity * time.delta_seconds();
        params.pitch = params.pitch.clamp(-89.5, 89.5);

        // Apply our rotation to the transform.
        transform.rotation = Quat::from_axis_angle(Vec3::Y, params.yaw.to_radians())
            * Quat::from_axis_angle(-Vec3::X, params.pitch.to_radians());
    }
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut FlyCam, &mut Transform)>,
) {
    for (mut params, mut transform) in query.iter_mut() {
        // Get the keyboard input for the X and Z plane.
        let mut input = Vec3::new(
            -input_axis(&keyboard_input, KeyCode::A, KeyCode::D),
            0.0,
            -input_axis(&keyboard_input, KeyCode::W, KeyCode::S),
        )
        .normalize_or_zero();

        // Rotate the input before we apply the vertical input.
        input = transform.rotation.mul_vec3(input);
        input += Vec3::Y * input_axis(&keyboard_input, KeyCode::Space, KeyCode::LShift);

        // Accelerate our velocity to our input.
        let accel = params.acceleration;
        params.velocity += input * accel * time.delta_seconds();

        // STOP! You have violated the law. (Speeding, clamp it bitch!)
        if params.velocity.length() > params.top_speed {
            params.velocity = params.velocity.normalize() * params.top_speed;
        }

        // Only if the camera has some velocity, apply a linear reaction force then apply the delta time to it.
        let friction: Vec3 = if params.velocity.length() != 0.0 {
            params.velocity.normalize() * -1.0 * params.friction
        } else {
            Vec3::ZERO
        } * time.delta_seconds();

        // Make sure our friction/reaction force won't cause the camera to reverse before applying it.
        params.velocity = if (params.velocity + friction).signum() != params.velocity.signum() {
            Vec3::ZERO
        } else {
            params.velocity + friction
        };

        // Apply our velocity.
        transform.translation += params.velocity;
    }
}
fn input_axis(keyboard: &Res<Input<KeyCode>>, positive_key: KeyCode, negative_key: KeyCode) -> f32 {
    let mut val = 0.0;
    if keyboard.pressed(positive_key) {
        val += 1.0;
    }
    if keyboard.pressed(negative_key) {
        val -= 1.0;
    }
    val
}
