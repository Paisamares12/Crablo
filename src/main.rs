mod art;
mod game;
mod levels;

use art::*;
use art::{BLUE, LIME, PURPLE};
use game::{Game, Outcome};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
enum AppState {
    Menu,
    Playing,
    Paused,
    LevelClear,
    Victory,
    GameOver,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Crablo - La gran fuga".into(),
        window_width: 1280,
        window_height: 800,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 1,
        ..Default::default()
    }
}

fn menu(art: &Art, selected: &mut usize, fullscreen: &mut bool, mouse: Vec2, time: f32) -> bool {
    backdrop(time);
    text("CRABLO", 31., 56., 6., color_u8!(56, 69, 56, 255));
    text("CRABLO", 28., 51., 6., LIME);
    text("LA GRAN FUGA", 32., 106., 2., PURPLE);
    text("ELIGE TU PERSONAJE", 32., 158., 1., INK);

    for (i, character) in [2, 1].iter().enumerate() {
        let rect = Rect::new(32. + i as f32 * 130., 178., 118., 86.);
        let color = if *character == 2 { CORAL } else { BLUE };
        let chosen = selected == character;
        let hover = rect.contains(mouse);
        panel(
            rect,
            if chosen {
                color
            } else if hover {
                INK
            } else {
                EDGE
            },
        );
        art.actor(
            vec2(rect.x + 27., rect.y + 58.),
            *character,
            4,
            true,
            time,
            2.,
            WHITE,
        );
        text(
            if *character == 2 { "ROJO" } else { "AZUL" },
            rect.x + 57.,
            rect.y + 24.,
            1.,
            color,
        );
        text(
            if chosen { "> LISTO" } else { "ELEGIR" },
            rect.x + 57.,
            rect.y + 44.,
            1.,
            if chosen { INK } else { MUTED },
        );
        if hover && is_mouse_button_pressed(MouseButton::Left) {
            *selected = *character;
        }
    }
    if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Key1)
    {
        *selected = 2;
    }
    if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Key2)
    {
        *selected = 1;
    }
    let start = button(Rect::new(32., 287., 248., 31.), "JUGAR", mouse, LIME);
    centered(
        "ENTER PARA JUGAR   /   1 ROJO - 2 AZUL",
        156.,
        335.,
        1.,
        MUTED,
    );

    // A small animated diorama built from the same tiles and sprites as the game.
    let origin = vec2(468., 168. + (time * 0.8).sin() * 3.);
    for depth in 0..13 {
        for y in 0..7 {
            if depth < y || depth - y >= 7 {
                continue;
            }
            let x = depth - y;
            let p = iso((x, y), origin);
            art.tile(p, 0, true, 1.);
            if (x, y) == (0, 1) || (x, y) == (5, 0) || (x, y) == (6, 5) {
                art.tile(p - vec2(0., 18.), 0, true, 1.);
            }
            if (x, y) == (2, 2) || (x, y) == (4, 4) {
                diamond(p - vec2(0., 23.), 3., 5., GOLD);
            }
            if (x, y) == (3, 3) {
                art.actor(p - vec2(0., 18.), *selected, 4, true, time, 2., WHITE);
            }
            if (x, y) == (1, 4) || (x, y) == (5, 2) {
                art.actor(p - vec2(0., 18.), 0, 4, true, time + 0.5, 1., WHITE);
            }
        }
    }
    if button(
        Rect::new(352., 320., 250., 24.),
        if *fullscreen {
            "MODO VENTANA"
        } else {
            "PANTALLA COMPLETA"
        },
        mouse,
        MUTED,
    ) {
        *fullscreen = !*fullscreen;
        set_fullscreen(*fullscreen);
    }
    draw_line(32., 377., 608., 377., 1., EDGE);
    text("CLIC PARA CAMINAR Y ATACAR", 32., 386., 1., MUTED);
    text("F11: PANTALLA COMPLETA", 464., 386., 1., MUTED);
    start || is_key_pressed(KeyCode::Enter)
}

fn overlay(title: &str, detail: &str, color: Color) {
    draw_rectangle(0., 0., W, H, Color::new(0.02, 0.03, 0.07, 0.87));
    panel(Rect::new(124., 105., 392., 192.), color);
    centered(title, 320., 151., 3., color);
    centered(detail, 320., 191., 1., INK);
}

#[macroquad::main(window_conf)]
async fn main() {
    let art = Art::new();
    let target = render_target(W as u32, H as u32);
    target.texture.set_filter(FilterMode::Nearest);
    let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., W, H));
    camera.render_target = Some(target.clone());
    let mut state = AppState::Menu;
    let mut selected = 2;
    let mut game = Game::new(selected);
    let mut fullscreen = window_conf().fullscreen;

    loop {
        if is_key_pressed(KeyCode::F11) {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen);
        }
        // Fit the available space without rounding down to a smaller integer scale.
        // Use the same transform for rendering and mouse input.
        let scale = (screen_width() / W).min(screen_height() / H).max(0.1);
        let offset = vec2(
            (screen_width() - W * scale) / 2.,
            (screen_height() - H * scale) / 2.,
        );
        let mouse = (Vec2::from(mouse_position()) - offset) / scale;
        let time = get_time() as f32;
        set_camera(&camera);
        match state {
            AppState::Menu => {
                if menu(&art, &mut selected, &mut fullscreen, mouse, time) {
                    game = Game::new(selected);
                    state = AppState::Playing;
                }
            }
            AppState::Playing => {
                if is_key_pressed(KeyCode::Escape) {
                    state = AppState::Paused;
                } else {
                    match game.update(get_frame_time(), mouse) {
                        Outcome::Defeat => state = AppState::GameOver,
                        Outcome::Cleared => {
                            state = if game.level_index + 1 == levels::LEVEL_COUNT {
                                AppState::Victory
                            } else {
                                AppState::LevelClear
                            }
                        }
                        Outcome::Running => {}
                    }
                }
                game.draw(&art, mouse, time);
            }
            AppState::Paused => {
                game.draw(&art, vec2(-100., -100.), time);
                overlay("PAUSA", "", PURPLE);
                if button(
                    Rect::new(190., 224., 260., 24.),
                    "CONTINUAR  /  ESC",
                    mouse,
                    LIME,
                ) || is_key_pressed(KeyCode::Escape)
                {
                    state = AppState::Playing;
                }
                if button(
                    Rect::new(190., 260., 260., 20.),
                    "VOLVER AL INICIO",
                    mouse,
                    MUTED,
                ) {
                    state = AppState::Menu;
                }
            }
            AppState::LevelClear => {
                game.draw(&art, vec2(-100., -100.), time);
                overlay(
                    "SECTOR LIBRE",
                    &format!("SIGUIENTE: {}", levels::level(game.level_index + 1).name),
                    LIME,
                );
                centered("+25 VIDA   /   +250 PUNTOS", 320., 213., 1., GOLD);
                if button(
                    Rect::new(190., 242., 260., 28.),
                    "SIGUIENTE NIVEL  /  ENTER",
                    mouse,
                    LIME,
                ) || is_key_pressed(KeyCode::Enter)
                {
                    game.advance();
                    state = AppState::Playing;
                }
            }
            AppState::Victory | AppState::GameOver => {
                game.draw(&art, vec2(-100., -100.), time);
                let victory = matches!(state, AppState::Victory);
                overlay(
                    if victory { "LIBERTAD!" } else { "TE ATRAPARON" },
                    if victory {
                        "Completaste todos los niveles."
                    } else {
                        ""
                    },
                    if victory { LIME } else { CORAL },
                );
                centered(
                    &format!(
                        "PUNTOS {:05}  /  SECTOR {:02}",
                        game.score,
                        game.level_index + 1
                    ),
                    320.,
                    214.,
                    1.,
                    GOLD,
                );
                if button(
                    Rect::new(190., 242., 260., 28.),
                    "VOLVER AL INICIO  /  ENTER",
                    mouse,
                    if victory { LIME } else { CORAL },
                ) || is_key_pressed(KeyCode::Enter)
                {
                    state = AppState::Menu;
                }
            }
        }
        set_default_camera();
        clear_background(BG);
        draw_texture_ex(
            &target.texture,
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(W, H) * scale),
                flip_y: true,
                ..Default::default()
            },
        );
        next_frame().await;
    }
}
