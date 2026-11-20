use macroquad::prelude::*;

const MIN_ZOOM: f32 = 0.25;
const MAX_ZOOM: f32 = 4.0;
const ZOOM_STEP: f32 = 1.1;

/// A simple 2D pan/zoom camera.
///
/// `center` is the world point shown at the middle of the screen and
/// `zoom` is the number of screen pixels per world unit.
pub struct PanCamera {
    pub center: Vec2,
    pub zoom: f32,
}

impl Default for PanCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl PanCamera {
    pub fn new() -> Self {
        Self {
            center: Vec2::ZERO,
            zoom: 1.0,
        }
    }

    /// Puts the camera back at the world origin with no zoom.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    fn screen_mid() -> Vec2 {
        vec2(screen_width(), screen_height()) / 2.0
    }

    pub fn world_to_screen(&self, world: Vec2) -> Vec2 {
        (world - self.center) * self.zoom + Self::screen_mid()
    }

    pub fn screen_to_world(&self, screen: Vec2) -> Vec2 {
        (screen - Self::screen_mid()) / self.zoom + self.center
    }

    /// The part of the world currently visible on screen.
    pub fn visible_world_rect(&self) -> Rect {
        let top_left = self.screen_to_world(vec2(0.0, 0.0));
        let bottom_right = self.screen_to_world(vec2(screen_width(), screen_height()));
        Rect::new(
            top_left.x,
            top_left.y,
            bottom_right.x - top_left.x,
            bottom_right.y - top_left.y,
        )
    }

    /// Pans by a mouse movement in screen pixels.
    ///
    /// The camera moves opposite to the mouse (drag right, see what's on the
    /// left), and dividing by `zoom` keeps the world stuck to the cursor at
    /// any zoom level.
    pub fn pan(&mut self, screen_delta: Vec2) {
        self.center -= screen_delta / self.zoom;
    }

    /// Zooms in or out around a point on screen (usually the cursor), so the
    /// world position under that point stays where it is.
    pub fn zoom_at(&mut self, screen_point: Vec2, wheel_y: f32) {
        if wheel_y == 0.0 {
            return;
        }
        let before = self.screen_to_world(screen_point);
        self.zoom = (self.zoom * ZOOM_STEP.powf(wheel_y.signum())).clamp(MIN_ZOOM, MAX_ZOOM);
        let after = self.screen_to_world(screen_point);
        self.center += before - after;
    }
}
