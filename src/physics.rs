use rapier2d::prelude::*;

/// The rapier2d simulation for the field.
///
/// Gravity is zero: this is a top-down robot on the ground, so the only
/// forces that matter come from the driving input and wall contacts.
pub struct Physics {
    pub world: PhysicsWorld,
}

impl Default for Physics {
    fn default() -> Self {
        Self::new()
    }
}

impl Physics {
    pub fn new() -> Self {
        let mut world = PhysicsWorld::new();
        world.gravity = Vector::ZERO;
        Self { world }
    }

    /// Adds an immovable collider for the outline through `points` (world
    /// coordinates, matching the drawn outline).
    ///
    /// Each edge becomes its own segment collider on a shared fixed body.
    /// Per-edge segments rather than one polyline collider because rapier
    /// 0.36 builds unreliable contact manifolds between a moving cuboid and
    /// small closed polyline loops, letting the robot push through them.
    pub fn add_static_polyline(&mut self, points: &[(f64, f64)]) {
        if points.len() < 2 {
            return;
        }

        let vertices: Vec<Vector> = points
            .iter()
            .map(|&(x, y)| Vector::new(x as f32, y as f32))
            .collect();

        let (body, _) = self.world.insert(
            RigidBodyBuilder::fixed(),
            ColliderBuilder::segment(vertices[0], vertices[1])
                .friction(0.8)
                .restitution(0.0),
        );
        for i in 1..vertices.len() {
            let a = vertices[i];
            let b = vertices[(i + 1) % vertices.len()];
            self.world.insert_collider(
                ColliderBuilder::segment(a, b)
                    .friction(0.8)
                    .restitution(0.0),
                Some(body),
            );
        }
    }

    /// Advances the simulation by `dt`. Clamping keeps a hitch from
    /// teleporting the robot through thin walls.
    pub fn step(&mut self, dt: f32) {
        self.world.integration_parameters.dt = dt.min(1.0 / 20.0);
        self.world.step();
    }
}

#[cfg(test)]
mod tests {







    use super::*;

    fn square_ring(half: f64) -> Vec<(f64, f64)> {
        vec![(half, half), (-half, half), (-half, -half), (half, -half)]
    }

    #[test]
    fn robot_stops_against_wall() {
        let mut physics = Physics::new();
        physics.add_static_polyline(&square_ring(700.0));

        let (robot, _) = physics.world.insert(
            RigidBodyBuilder::dynamic().translation(Vector::new(0.0, 0.0)),
            ColliderBuilder::cuboid(90.0, 90.0).friction(0.4),
        );

        // Drive straight right at MOVE_SPEED for 100 simulated seconds.
        for _ in 0..6000 {
            physics.world.bodies[robot].set_linvel(Vector::new(300.0, 0.0), true);
            physics.step(1.0 / 60.0);
        }

        let x = physics.world.bodies[robot].translation().x;
        // Wall line at x = 700, robot half-size 90: it must stop near 610,
        // not end up outside or far inside the wall.
        assert!(x < 700.0, "robot passed through the wall: x = {x}");
        assert!(x > 580.0, "robot stopped far from the wall: x = {x}");
    }

    #[test]
    fn robot_stops_before_center_box() {
        let mut physics = Physics::new();
        physics.add_static_polyline(&square_ring(700.0));
        physics.add_static_polyline(&square_ring(28.1));

        let (robot, _) = physics.world.insert(
            RigidBodyBuilder::dynamic().translation(Vector::new(0.0, 200.0)),
            ColliderBuilder::cuboid(90.0, 90.0).friction(0.4),
        );

        // Drive in -y toward the center box.
        for _ in 0..6000 {
            physics.world.bodies[robot].set_linvel(Vector::new(0.0, -300.0), true);
            physics.step(1.0 / 60.0);
        }

        let y = physics.world.bodies[robot].translation().y;
        // Box top edge at y = 28.1, robot half-size 90: it must stop near 118.
        // Driving straight through would leave y well below zero.
        assert!(y > 110.0, "robot drove through the center box: y = {y}");
        assert!(y < 119.5, "robot stopped far from the center box: y = {y}");
    }
}
