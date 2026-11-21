mod camera;
mod field;

use camera::PanCamera;
use field::Field;
use macroquad::prelude::*;

const GRID_STEP: f32 = 100.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Autofield".to_owned(),
        window_width: 960,
        window_height: 640,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let field = Field::from_str(include_str!("../assets/fields/field.json"))
        .expect("assets/field.json is not valid");
    let mut cam = PanCamera::new();
    let mut last_mouse: Vec2 = mouse_position().into();

    loop {
        let mouse: Vec2 = mouse_position().into();
        let delta = mouse - last_mouse;
        last_mouse = mouse;

        // --- Input -----------------------------------------------------------
        if is_mouse_button_down(MouseButton::Left) {
            cam.pan(delta);
        }

        cam.zoom_at(mouse, mouse_wheel().1);

        if is_key_pressed(KeyCode::R) {
            cam.reset();
        }

        // --- Drawing ---------------------------------------------------------
        clear_background(Color::from_rgba(24, 28, 36, 255));

        draw_grid(&cam);
        field.render(&cam);

        let world_mouse = cam.screen_to_world(mouse);
        draw_text(
            format!(
                "Camera: ({:.0}, {:.0})   Zoom: {:.2}x   Cursor in world: ({:.0}, {:.0})",
                cam.center.x, cam.center.y, cam.zoom, world_mouse.x, world_mouse.y,
            ),
            16.0,
            screen_height() - 16.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}

fn draw_grid(cam: &PanCamera) {
    let (sw, sh) = (screen_width(), screen_height());
    let view = cam.visible_world_rect();

    let minor = Color::new(1.0, 1.0, 1.0, 0.07);
    let axis = Color::new(1.0, 1.0, 1.0, 0.35);

    // Vertical lines
    let mut x = (view.x / GRID_STEP).floor() * GRID_STEP;
    while x <= view.x + view.w {
        let sx = cam.world_to_screen(vec2(x, 0.0)).x;
        let (color, w) = if x == 0.0 { (axis, 2.0) } else { (minor, 1.0) };
        draw_line(sx, 0.0, sx, sh, w, color);
        x += GRID_STEP;
    }

    // Horizontal lines
    let mut y = (view.y / GRID_STEP).floor() * GRID_STEP;
    while y <= view.y + view.h {
        let sy = cam.world_to_screen(vec2(0.0, y)).y;
        let (color, w) = if y == 0.0 { (axis, 2.0) } else { (minor, 1.0) };
        draw_line(0.0, sy, sw, sy, w, color);
        y += GRID_STEP;
    }
}
