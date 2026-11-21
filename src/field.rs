use crate::camera::PanCamera;
use macroquad::prelude::*;
use serde::Deserialize;
use serde_json::{Error, from_str};
use std::f32::consts::PI;

const HARD_OUTLINE: Color = Color::new(0.95, 0.60, 0.25, 1.0);
const SOFT_OUTLINE: Color = Color::new(0.30, 0.75, 0.45, 0.90);

#[derive(Deserialize)]
pub struct Field {
    hard_props: Vec<Prop>,
    soft_props: Vec<Prop>,
}

impl Field {
    pub fn from_str(file: &str) -> Result<Self, Error> {
        from_str(file)
    }

    /// Draws every prop through the camera. Soft props go first so the hard
    /// props are always drawn on top of them.
    pub fn render(&self, cam: &PanCamera) {
        for prop in &self.soft_props {
            prop.render(cam, SOFT_OUTLINE, 2.0);
        }
        for prop in &self.hard_props {
            prop.render(cam, HARD_OUTLINE, 3.0);
        }
    }
}

#[derive(Deserialize)]
pub struct Prop {
    /// X, Y and Roundness
    points: Vec<(f64, f64, f64)>,
}

impl Prop {
    fn render(&self, cam: &PanCamera, color: Color, thickness: f32) {
        if self.points.len() < 3 {
            return;
        }

        // World space -> screen space. The radius scales with zoom so the
        // rounding looks the same at any zoom level.
        let corners: Vec<Corner> = self
            .points
            .iter()
            .map(|&(x, y, r)| Corner {
                pos: cam.world_to_screen(vec2(x as f32, y as f32)),
                radius: r as f32 * cam.zoom,
            })
            .collect();

        // Skip props that are completely off-screen
        let (mut min, mut max) = (corners[0].pos, corners[0].pos);
        for c in &corners {
            min = min.min(c.pos);
            max = max.max(c.pos);
        }
        if max.x < 0.0 || max.y < 0.0 || min.x > screen_width() || min.y > screen_height() {
            return;
        }

        // Bigger rounded corners on screen get more segments
        let max_radius = corners.iter().map(|c| c.radius).fold(0.0, f32::max);
        let segments = ((max_radius / 4.0) as usize).clamp(4, 24);

        let points = rounded_outline(&corners, segments);
        draw_outline(&points, color, thickness);
    }
}

/// A polygon corner with its own rounding radius (0.0 = sharp).
#[derive(Clone, Copy)]
struct Corner {
    pos: Vec2,
    radius: f32,
}

/// Draws a closed polygon as lines only.
///
/// Because nothing is filled, convex and concave shapes both work.
fn draw_outline(points: &[Vec2], color: Color, thickness: f32) {
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        draw_line(a.x, a.y, b.x, b.y, thickness, color);
    }
}

/// Replaces each corner with an arc of its own radius and returns the
/// resulting list of points. `segments` is how many pieces each arc has.
fn rounded_outline(corners: &[Corner], segments: usize) -> Vec<Vec2> {
    let n = corners.len();
    let mut out = Vec::new();

    for i in 0..n {
        let prev = corners[(i + n - 1) % n].pos;
        let cur = corners[i];
        let next = corners[(i + 1) % n].pos;

        let to_prev = prev - cur.pos;
        let to_next = next - cur.pos;
        let (len_p, len_n) = (to_prev.length(), to_next.length());

        // Sharp corner or degenerate edge: keep the point as it is
        if cur.radius <= 0.0 || len_p < 1e-4 || len_n < 1e-4 {
            out.push(cur.pos);
            continue;
        }

        let u = to_prev / len_p;
        let v = to_next / len_n;
        let theta = u.dot(v).clamp(-1.0, 1.0).acos(); // angle between the edges

        // Edges nearly collinear or folded back: nothing to round
        if theta < 1e-3 || PI - theta < 1e-3 {
            out.push(cur.pos);
            continue;
        }

        // Distance from the corner to the arc's tangent points, clamped so
        // neighbouring arcs never overlap.
        let half_tan = (theta / 2.0).tan();
        let d = (cur.radius / half_tan).min(len_p / 2.0).min(len_n / 2.0);
        let r = d * half_tan; // actual radius after clamping

        let center = cur.pos + (u + v).normalize() * (r / (theta / 2.0).sin());
        let start = (cur.pos + u * d) - center;
        let end = (cur.pos + v * d) - center;

        let a0 = start.y.atan2(start.x);
        let mut sweep = end.y.atan2(end.x) - a0;
        while sweep > PI {
            sweep -= 2.0 * PI; // take the short way round
        }
        while sweep < -PI {
            sweep += 2.0 * PI;
        }

        for s in 0..=segments {
            let a = a0 + sweep * (s as f32 / segments as f32);
            out.push(center + vec2(a.cos(), a.sin()) * r);
        }
    }
    out
}
