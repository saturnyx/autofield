use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

const PANEL_POS: Vec2 = vec2(16.0, 44.0);
const PANEL_SIZE: Vec2 = vec2(320.0, 268.0);

/// Live-tunable driving and physics constants, edited with the in-game
/// panel (T toggles its visibility).
#[derive(Debug, Clone, Copy)]
pub struct Tuning {
    pub visible: bool,
    pub drive_speed: f32,
    pub turn_speed: f32,
    pub stick_deadzone: f32,
    pub restitution: f32,
    pub time_scale: f32,
}

impl Default for Tuning {
    fn default() -> Self {
        Self {
            visible: true,
            drive_speed: 3000.0,
            turn_speed: 2.5,
            stick_deadzone: 0.15,
            restitution: 0.0,
            time_scale: 1.0,
        }
    }
}

impl Tuning {
    pub fn toggle(&mut self) {
        if is_key_pressed(KeyCode::T) {
            self.visible = !self.visible;
        }
    }

    /// Is this screen-space point over the panel? The main loop uses it to
    /// keep camera panning/zooming from activating while a slider is used.
    pub fn panel_contains(&self, p: Vec2) -> bool {
        self.visible
            && p.x >= PANEL_POS.x
            && p.x <= PANEL_POS.x + PANEL_SIZE.x
            && p.y >= PANEL_POS.y
            && p.y <= PANEL_POS.y + PANEL_SIZE.y
    }

    pub fn draw(&mut self) {
        if !self.visible {
            return;
        }
        root_ui().window(hash!(), PANEL_POS, PANEL_SIZE, |ui| {
            ui.label(None, "Tuning  (T to hide)");
            ui.label(None, &format!("Drive speed: {:.0}", self.drive_speed));
            ui.slider(hash!(), "drive speed", 0.0..6000.0, &mut self.drive_speed);
            ui.label(None, &format!("Turn speed: {:.2} rad/s", self.turn_speed));
            ui.slider(hash!(), "turn rad/s", 0.0..8.0, &mut self.turn_speed);
            ui.label(None, &format!("Stick deadzone: {:.2}", self.stick_deadzone));
            ui.slider(hash!(), "deadzone", 0.0..0.5, &mut self.stick_deadzone);
            ui.label(None, &format!("Wall bounce: {:.2}", self.restitution));
            ui.slider(hash!(), "bounce", 0.0..1.0, &mut self.restitution);
            ui.label(None, &format!("Time scale: {:.2}x", self.time_scale));
            ui.slider(hash!(), "time", 0.1..2.0, &mut self.time_scale);
        });
    }
}
