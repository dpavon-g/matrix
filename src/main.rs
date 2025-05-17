use macroquad::prelude::*;

pub struct ScreenSettings {
    pub screen_w: f32,
    pub screen_h: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub scale: f32,
    pub spacing: f32,
    pub palette: Palette
}

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub thickness: f32,
}

pub struct Palette {
    pub blanco: Color,
    pub gris_claro: Color,
    pub gris: Color,
    pub gris_oscuro: Color,
    pub negro: Color,

    pub azul: Color,
    pub azul_claro: Color,
    pub azul_oscuro: Color,

    pub rojo: Color,
    pub rojo_claro: Color,
    pub rojo_oscuro: Color,

    pub verde: Color,
    pub verde_claro: Color,
    pub verde_oscuro: Color,
}

impl Palette {
    pub fn new() -> Self {
        Self {
            blanco: Color::from_rgba(242, 242, 242, 255),
            gris_claro: Color::from_rgba(191, 191, 191, 255),
            gris: Color::from_rgba(140, 140, 140, 255),
            gris_oscuro: Color::from_rgba(63, 63, 63, 255),
            negro: Color::from_rgba(12, 12, 12, 255),

            azul: Color::from_rgba(77, 166, 255, 255),       // #4DA6FF
            azul_claro: Color::from_rgba(153, 204, 255, 255),// #99CCFF
            azul_oscuro: Color::from_rgba(0, 89, 179, 255),  // #0059B3

            rojo: Color::from_rgba(255, 102, 102, 255),      // #FF6666
            rojo_claro: Color::from_rgba(255, 153, 153, 255),// #FF9999
            rojo_oscuro: Color::from_rgba(179, 36, 36, 255), // #B32424

            verde: Color::from_rgba(102, 255, 153, 255),     // #66FF99
            verde_claro: Color::from_rgba(153, 255, 204, 255),// #99FFCC
            verde_oscuro: Color::from_rgba(31, 153, 102, 255),// #1F9966
        }
    }
}

async fn draw_vertical_lines(screen_settings: &ScreenSettings) {
    let grid_size = screen_settings.spacing * 20.0;
    let half_grid_size = grid_size / 2.0;
    let mut y = screen_settings.center_y;

    let mut total_lines = 0;

    while total_lines <= 10 {
        draw_line(screen_settings.center_x - half_grid_size, y, screen_settings.center_x + half_grid_size, y, 1.5, screen_settings.palette.gris);
        y += screen_settings.spacing;
        total_lines += 1;
    }
    y = screen_settings.center_y;
    total_lines = 0;
    while total_lines <= 10 {
        draw_line(screen_settings.center_x - half_grid_size, y, screen_settings.center_x + half_grid_size, y, 1.5, screen_settings.palette.gris);
        y -= screen_settings.spacing;
        total_lines += 1;
    }
}

async fn draw_horizontal_lines(screen_settings: &ScreenSettings) {
    let grid_size = screen_settings.spacing * 20.0;
    let half_grid_size = grid_size / 2.0;
    let mut x = screen_settings.center_x;

    let mut total_lines = 0;

    while total_lines <= 10 {
        draw_line(x, screen_settings.center_y - half_grid_size, x, screen_settings.center_y + half_grid_size, 1.5, screen_settings.palette.gris);
        x += screen_settings.spacing;
        total_lines += 1;
    }
    x = screen_settings.center_x;
    total_lines = 0;
    while total_lines <= 10 {
        draw_line(x, screen_settings.center_y - half_grid_size, x, screen_settings.center_y + half_grid_size, 1.5, screen_settings.palette.gris);
        x -= screen_settings.spacing;
        total_lines += 1;
    }
}

async fn draw_new_grid(screen_settings: &mut ScreenSettings) {
    screen_settings.spacing = 25.0 * screen_settings.scale;
    draw_horizontal_lines(screen_settings).await;
    draw_vertical_lines(screen_settings).await;
}

async fn draw_vector(vector: Vector, screen_settings: &ScreenSettings) {
    draw_line(
        screen_settings.center_x,
        screen_settings.center_y,
        screen_settings.center_x + vector.x * screen_settings.spacing,
        screen_settings.center_y - vector.y * screen_settings.spacing,
        vector.thickness * screen_settings.scale,
        vector.color,
    );
}

async fn show_visual_vectors(vectores: [Vector; 3]) {
let palette = Palette::new();
    let mut screen_settings = ScreenSettings {
        screen_w: screen_width(),
        screen_h: screen_height(),
        center_x: screen_width() / 2.0,
        center_y: screen_height() / 2.0,
        scale: 1.0,
        spacing: 25.0,
        palette
    };

    loop {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }
        if is_key_pressed(KeyCode::Minus) {
            if (screen_settings.scale - 0.5) > 0.0 {
                screen_settings.scale -= 0.5;
            }
        }
        if is_key_pressed(KeyCode::Equal) {
            screen_settings.scale += 0.5;
        }
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            screen_settings.center_y += 5.0 * screen_settings.scale;
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            screen_settings.center_y -= 5.0 * screen_settings.scale;
        }
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            screen_settings.center_x += 5.0 * screen_settings.scale;
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            screen_settings.center_x -= 5.0 * screen_settings.scale;
        }
        clear_background(screen_settings.palette.gris_oscuro);
        draw_new_grid(&mut screen_settings).await;
       
        for vector in &vectores {
            draw_vector(*vector, &screen_settings).await;
        }
        draw_circle(screen_settings.center_x, screen_settings.center_y, 4.0 * screen_settings.scale, screen_settings.palette.rojo);
        next_frame().await;
    }
}
#[macroquad::main("Vector Renderer")]
async fn main() {
    let palette = Palette::new();
    let vectores: [Vector; 3] = [
        Vector { x: 1.0, y: 4.0,  color: palette.rojo, thickness: 3.0 },
        Vector { x: 1.0, y: 3.0, color: palette.azul, thickness: 3.0 },
        Vector { x: 1.0, y: 2.0, color: palette.verde, thickness: 3.0 }
    ];
    show_visual_vectors(vectores).await;
}
