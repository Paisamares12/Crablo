use crate::levels::Cell;
use macroquad::prelude::*;

pub const W: f32 = 640.;
pub const H: f32 = 400.;
pub const BG: Color = color_u8!(15, 19, 34, 255);
pub const PANEL: Color = color_u8!(25, 31, 49, 255);
pub const EDGE: Color = color_u8!(54, 65, 85, 255);
pub const INK: Color = color_u8!(232, 236, 220, 255);
pub const MUTED: Color = color_u8!(139, 155, 171, 255);
pub const LIME: Color = color_u8!(191, 235, 117, 255);
pub const PURPLE: Color = color_u8!(173, 151, 232, 255);
pub const CORAL: Color = color_u8!(240, 133, 115, 255);
pub const BLUE: Color = color_u8!(116, 203, 232, 255);

pub struct Art {
    characters: Texture2D,
    tiles: Texture2D,
}

impl Art {
    pub fn new() -> Self {
        let characters =
            Texture2D::from_file_with_format(include_bytes!("../assets/characters.png"), None);
        // The reference is a screenshot: key out its neutral gray backdrop in memory.
        let mut image = Image::from_file_with_format(include_bytes!("../assets/tiles.png"), None)
            .expect("Invalid embedded tileset");
        let key = image.get_pixel(4, 20);
        let rgb = [
            (key.r * 255.) as u8,
            (key.g * 255.) as u8,
            (key.b * 255.) as u8,
        ];
        for pixel in image.bytes.chunks_exact_mut(4) {
            if pixel[..3].iter().zip(rgb).all(|(a, b)| a.abs_diff(b) <= 4) {
                pixel[3] = 0;
            }
        }
        let tiles = Texture2D::from_image(&image);
        characters.set_filter(FilterMode::Nearest);
        tiles.set_filter(FilterMode::Nearest);
        Self { characters, tiles }
    }

    pub fn tile(&self, pos: Vec2, theme: usize, wall: bool, scale: f32) {
        let offset = [0., 208., 416.][theme % 3];
        let (source, size, top) = if wall {
            (Rect::new(60., 45. + offset, 69., 70.), vec2(36., 36.), 27.)
        } else {
            (Rect::new(130., 148. + offset, 68., 35.), vec2(36., 18.), 9.)
        };
        draw_texture_ex(
            &self.tiles,
            (pos.x - 18. * scale).round(),
            (pos.y - top * scale).round(),
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(size * scale),
                ..Default::default()
            },
        );
    }

    /// The sheet has eight directions and three 24px animation rows per color.
    pub fn actor(
        &self,
        pos: Vec2,
        kind: usize,
        direction: usize,
        walking: bool,
        time: f32,
        scale: f32,
        tint: Color,
    ) {
        let frame = if walking {
            [0, 1, 2, 1][(time * 8.) as usize % 4]
        } else {
            0
        };
        let row = [24, 120, 216][kind] + frame * 24;
        draw_ellipse(
            pos.x,
            pos.y + 2. * scale,
            6. * scale,
            2. * scale,
            0.,
            Color::new(0., 0., 0., 0.3),
        );
        draw_texture_ex(
            &self.characters,
            (pos.x - 8. * scale).round(),
            (pos.y - 21. * scale).round(),
            tint,
            DrawTextureParams {
                source: Some(Rect::new((direction % 8 * 16) as f32, row as f32, 16., 24.)),
                dest_size: Some(vec2(16., 24.) * scale),
                ..Default::default()
            },
        );
    }
}

pub fn iso(pos: Cell, cam: Vec2) -> Vec2 {
    vec2(
        (pos.0 as f32 - pos.1 as f32) * 18.,
        (pos.0 as f32 + pos.1 as f32) * 9.,
    ) + cam
}

pub fn pick(point: Vec2, cam: Vec2) -> Option<Cell> {
    let p = point - cam;
    let x = ((p.x / 18. + p.y / 9.) / 2.).round() as i32;
    let y = ((p.y / 9. - p.x / 18.) / 2.).round() as i32;
    (x >= 0 && y >= 0 && x < crate::levels::MAP as i32 && y < crate::levels::MAP as i32)
        .then_some((x as usize, y as usize))
}

pub fn diamond(p: Vec2, width: f32, height: f32, color: Color) {
    let a = p + vec2(0., -height);
    let b = p + vec2(width, 0.);
    let c = p + vec2(0., height);
    let d = p + vec2(-width, 0.);
    draw_triangle(a, b, c, color);
    draw_triangle(a, c, d, color);
}

pub fn panel(rect: Rect, accent: Color) {
    draw_rectangle(rect.x + 3., rect.y + 4., rect.w, rect.h, BLACK);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, accent);
    draw_rectangle(rect.x + 1., rect.y + 1., rect.w - 2., rect.h - 2., PANEL);
    for (x, y) in [
        (rect.x, rect.y),
        (rect.x + rect.w - 2., rect.y),
        (rect.x, rect.y + rect.h - 2.),
        (rect.x + rect.w - 2., rect.y + rect.h - 2.),
    ] {
        draw_rectangle(x, y, 2., 2., BG);
    }
}

pub fn button(rect: Rect, label: &str, mouse: Vec2, accent: Color) -> bool {
    let hover = rect.contains(mouse);
    draw_rectangle(rect.x + 2., rect.y + 3., rect.w, rect.h, BLACK);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if hover { INK } else { accent },
    );
    centered(
        label,
        rect.x + rect.w / 2.,
        rect.y + (rect.h - 7.) / 2.,
        1.,
        BG,
    );
    hover && is_mouse_button_pressed(MouseButton::Left)
}

pub fn text_width(s: &str, scale: f32) -> f32 {
    s.chars().count() as f32 * 6. * scale - scale
}
pub fn centered(s: &str, x: f32, y: f32, scale: f32, color: Color) {
    text(s, x - text_width(s, scale) / 2., y, scale, color);
}

pub fn text(s: &str, x: f32, y: f32, scale: f32, color: Color) {
    for (i, ch) in s.to_uppercase().chars().enumerate() {
        let rows = glyph(ch);
        for (row, bits) in rows.iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    draw_rectangle(
                        (x + (i * 6 + col) as f32 * scale).round(),
                        (y + row as f32 * scale).round(),
                        scale,
                        scale,
                        color,
                    );
                }
            }
        }
    }
}

fn glyph(c: char) -> [u8; 7] {
    match c {
        'A' | 'Á' => [14, 17, 17, 31, 17, 17, 17],
        'B' => [30, 17, 17, 30, 17, 17, 30],
        'C' => [14, 17, 16, 16, 16, 17, 14],
        'D' => [30, 17, 17, 17, 17, 17, 30],
        'E' | 'É' => [31, 16, 16, 30, 16, 16, 31],
        'F' => [31, 16, 16, 30, 16, 16, 16],
        'G' => [14, 17, 16, 23, 17, 17, 15],
        'H' => [17, 17, 17, 31, 17, 17, 17],
        'I' | 'Í' => [14, 4, 4, 4, 4, 4, 14],
        'J' => [7, 2, 2, 2, 18, 18, 12],
        'K' => [17, 18, 20, 24, 20, 18, 17],
        'L' => [16, 16, 16, 16, 16, 16, 31],
        'M' => [17, 27, 21, 21, 17, 17, 17],
        'N' | 'Ñ' => [17, 25, 25, 21, 19, 19, 17],
        'O' | 'Ó' => [14, 17, 17, 17, 17, 17, 14],
        'P' => [30, 17, 17, 30, 16, 16, 16],
        'Q' => [14, 17, 17, 17, 21, 18, 13],
        'R' => [30, 17, 17, 30, 20, 18, 17],
        'S' => [15, 16, 16, 14, 1, 1, 30],
        'T' => [31, 4, 4, 4, 4, 4, 4],
        'U' | 'Ú' => [17, 17, 17, 17, 17, 17, 14],
        'V' => [17, 17, 17, 17, 17, 10, 4],
        'W' => [17, 17, 17, 21, 21, 21, 10],
        'X' => [17, 17, 10, 4, 10, 17, 17],
        'Y' => [17, 17, 10, 4, 4, 4, 4],
        'Z' => [31, 1, 2, 4, 8, 16, 31],
        '0' => [14, 17, 19, 21, 25, 17, 14],
        '1' => [4, 12, 4, 4, 4, 4, 14],
        '2' => [14, 17, 1, 2, 4, 8, 31],
        '3' => [30, 1, 1, 14, 1, 1, 30],
        '4' => [2, 6, 10, 18, 31, 2, 2],
        '5' => [31, 16, 16, 30, 1, 1, 30],
        '6' => [14, 16, 16, 30, 17, 17, 14],
        '7' => [31, 1, 2, 4, 8, 8, 8],
        '8' => [14, 17, 17, 14, 17, 17, 14],
        '9' => [14, 17, 17, 15, 1, 1, 14],
        '.' => [0, 0, 0, 0, 0, 12, 12],
        ',' => [0, 0, 0, 0, 0, 4, 8],
        ':' => [0, 12, 12, 0, 12, 12, 0],
        '-' => [0, 0, 0, 31, 0, 0, 0],
        '+' => [0, 4, 4, 31, 4, 4, 0],
        '/' => [1, 2, 2, 4, 8, 8, 16],
        '>' => [16, 8, 4, 2, 4, 8, 16],
        '<' => [1, 2, 4, 8, 4, 2, 1],
        '!' => [4, 4, 4, 4, 4, 0, 4],
        '?' => [14, 17, 1, 2, 4, 0, 4],
        '[' => [14, 8, 8, 8, 8, 8, 14],
        ']' => [14, 2, 2, 2, 2, 2, 14],
        _ => [0; 7],
    }
}

pub fn backdrop(time: f32) {
    clear_background(BG);
    for i in 0..55 {
        let x = ((i * 127 + 19) % 640) as f32;
        let y = ((i * 73 + 31) % 400) as f32;
        let c = if (time * 0.7 + i as f32).sin() > 0.7 {
            MUTED
        } else {
            EDGE
        };
        draw_rectangle(x, y, 1., 1., c);
    }
    for y in (0..400).step_by(4) {
        draw_line(0., y as f32, W, y as f32, 1., Color::new(0., 0., 0., 0.06));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn picking_round_trips_and_rejects_outside_map() {
        let cam = vec2(320., 89.);
        for y in 0..16 {
            for x in 0..16 {
                assert_eq!(pick(iso((x, y), cam), cam), Some((x, y)));
            }
        }
        assert_eq!(pick(vec2(320., 0.), cam), None);
        assert_eq!(pick(vec2(-100., 200.), cam), None);
    }
}
