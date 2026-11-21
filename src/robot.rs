use crate::camera::PanCamera;
use crate::physics::Physics;
use macroquad::prelude::*;
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder, RigidBodyHandle, Vector};

const SIZE: f32 = 180.0;
const MOVE_SPEED: f32 = 3000.0; // world units per second
const TURN_SPEED: f32 = 2.5;   // radians per second
const OUTLINE: Color = Color::new(0.35, 0.70, 1.00, 1.0);

/// A 180x180 differential-style robot: W/S drive it along its heading,
/// A/D rotate it in place. The pose lives on a dynamic rapier body, so the
/// robot slides along and stops against hard props instead of passing
/// through them.
pub struct Robot {
    pub pos: Vec2,
    pub heading: f32,
    body: RigidBodyHandle,
}

impl Robot {
    pub fn new(physics: &mut Physics) -> Self {
        let half = SIZE / 2.0;
        let (body, _) = physics.world.insert(
            RigidBodyBuilder::dynamic()
                .translation(Vector::new(0.0, 0.0))
                .rotation(0.0),
            ColliderBuilder::cuboid(half, half)
                .friction(0.4)
                .restitution(0.0),
        );
        Self {
            pos: Vec2::ZERO,
            heading: 0.0,
            body,
        }
    }

    /// Turns the driving keys into body velocities for this frame.
    pub fn drive(&mut self, physics: &mut Physics) {
        // A turns left (counter-clockwise on screen), D turns right.
        let mut turn = 0.0;
        if is_key_down(KeyCode::A) {
            turn -= TURN_SPEED;
        }
        if is_key_down(KeyCode::D) {
            turn += TURN_SPEED;
        }

        let mut drive = 0.0;
        if is_key_down(KeyCode::W) {
            drive += 1.0;
        }
        if is_key_down(KeyCode::S) {
            drive -= 1.0;
        }

        let velocity = vec2(self.heading.cos(), self.heading.sin()) * MOVE_SPEED * drive;
        let body = &mut physics.world.bodies[self.body];
        body.set_angvel(turn, true);
        body.set_linvel(Vector::new(velocity.x, velocity.y), true);
    }

    /// Copies the simulated pose back into the render state.
    pub fn sync(&mut self, physics: &Physics) {
        let body = &physics.world.bodies[self.body];
        let t = body.translation();
        self.pos = vec2(t.x, t.y);
        self.heading = body.rotation().angle();
    }

    /// Draws the square, its heading direction and a driver-side accent,
    /// all through the camera.
    pub fn render(&self, cam: &PanCamera) {
        let half = SIZE / 2.0;
        // f32::sin_cos returns (sin, cos)
        let (s, c) = self.heading.sin_cos();
        // Local corner offsets rotated by the heading
        let corners_local = [
            vec2(half, half),
            vec2(-half, half),
            vec2(-half, -half),
            vec2(half, -half),
        ];
        let to_world = |p: Vec2| {
            self.pos + vec2(p.x * c - p.y * s, p.x * s + p.y * c)
        };

        let corners: Vec<Vec2> = corners_local
            .iter()
            .map(|&p| cam.world_to_screen(to_world(p)))
            .collect();

        // Skip drawing when fully off-screen
        let (mut min, mut max) = (corners[0], corners[0]);
        for p in &corners {
            min = min.min(*p);
            max = max.max(*p);
        }
        if max.x < 0.0 || max.y < 0.0 || min.x > screen_width() || min.y > screen_height() {
            return;
        }

        let thickness = (2.0 * cam.zoom).max(1.0);
        for i in 0..corners.len() {
            let a = corners[i];
            let b = corners[(i + 1) % corners.len()];
            draw_line(a.x, a.y, b.x, b.y, thickness, OUTLINE);
        }

        // Heading arrow from the centre to the front edge midpoint
        let front_local = vec2(half, 0.0);
        let tip = cam.world_to_screen(to_world(front_local));
        let mid = cam.world_to_screen(self.pos);
        draw_line(mid.x, mid.y, tip.x, tip.y, thickness, OUTLINE);
    }
}
