use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

const BASE_POS: Vec2 = vec2(16.0, 44.0);
const PANEL_SIZE: Vec2 = vec2(420.0, 310.0);
/// Height of the strip used as the drag handle.
const TITLE_HEIGHT: f32 = 28.0;
/// The default skin with a larger font. Widget geometry (editbox widths,
/// slider heights) is hardcoded in macroquad widgets, so only text grows.
const FONT_SIZE: u16 = 22;

/// Live-tunable driving and physics constants, edited with the in-game
/// panel (T toggles its visibility, dragging the title strip moves it).
#[derive(Debug, Clone, Copy)]
pub struct Tuning {
    pub visible: bool,
    pub drive_speed: f32,
    pub turn_speed: f32,
    pub stick_deadzone: f32,
    pub restitution: f32,
    pub time_scale: f32,
    panel_pos: Vec2,
    drag: bool,
    drag_grab: Vec2,
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
            panel_pos: BASE_POS,
            drag: false,
            drag_grab: Vec2::ZERO,
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
        self.visible && Rect::new(self.pos().x, self.pos().y, PANEL_SIZE.x, PANEL_SIZE.y).contains(p)
    }

    fn clamp_pos(p: Vec2) -> Vec2 {
        // Keep the title strip reachable; allow the panel to hang off edges.
        p.clamp(
            vec2(60.0 - PANEL_SIZE.x, 0.0),
            vec2(screen_width() - 60.0, screen_height() - TITLE_HEIGHT),
        )
    }

    fn pos(&self) -> Vec2 {
        Self::clamp_pos(self.panel_pos)
    }

    pub fn draw(&mut self) {
        if !self.visible {
            return;
        }
        let mouse: Vec2 = mouse_position().into();

        if is_mouse_button_pressed(MouseButton::Left) && !self.drag {
            let titlebar = Rect::new(self.pos().x, self.pos().y, PANEL_SIZE.x, TITLE_HEIGHT);
            if titlebar.contains(mouse) {
                self.drag = true;
                self.drag_grab = mouse - self.pos();
            }
        }
        if self.drag {
            if is_mouse_button_down(MouseButton::Left) {
                self.panel_pos = Self::clamp_pos(mouse - self.drag_grab);
            } else {
                self.drag = false;
            }
        }

        let skin = build_skin();
        let pos = self.pos();
        let mut ui = root_ui();
        ui.push_skin(&skin);
        ui.window(hash!(), pos, PANEL_SIZE, |ui| {
            ui.label(None, "Tuning  (drag title, T to hide)");
            ui.label(None, &format!("Drive speed: {:.0}", self.drive_speed));
            ui.slider(hash!(), "", 0.0..6000.0, &mut self.drive_speed);
            ui.label(None, &format!("Turn speed: {:.2} rad/s", self.turn_speed));
            ui.slider(hash!(), "", 0.0..8.0, &mut self.turn_speed);
            ui.label(None, &format!("Stick deadzone: {:.2}", self.stick_deadzone));
            ui.slider(hash!(), "", 0.0..0.5, &mut self.stick_deadzone);
            ui.label(None, &format!("Wall bounce: {:.2}", self.restitution));
            ui.slider(hash!(), "", 0.0..1.0, &mut self.restitution);
            ui.label(None, &format!("Time scale: {:.2}x", self.time_scale));
            ui.slider(hash!(), "", 0.1..2.0, &mut self.time_scale);
        });
        ui.pop_skin();
    }
}

fn build_skin() -> Skin {
    let ui = root_ui();
    let mut skin = ui.default_skin();
    skin.label_style = ui.style_builder().font_size(FONT_SIZE).build();
    // The value box inside each slider is a hardcoded 50px wide, so its
    // font stays default-size; the trimmed value is still shown above it.
    skin.editbox_style = ui.style_builder().font_size(16).build();
    skin.button_style = ui.style_builder().font_size(FONT_SIZE).build();
    skin.window_style = ui.style_builder().font_size(FONT_SIZE).build();
    skin.window_titlebar_style = ui.style_builder().font_size(FONT_SIZE).build();
    skin
}
