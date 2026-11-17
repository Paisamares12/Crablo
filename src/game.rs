use crate::art::{BLUE, LIME, PURPLE};
use crate::{art::*, levels::*};
use macroquad::prelude::*;

pub const CAM: Vec2 = Vec2::new(320., 89.);
const ANNOUNCEMENT_INTERVAL: f32 = 6.;
const ANNOUNCEMENT_DURATION: f32 = 3.;
const ANNOUNCEMENTS: [&str; 6] = [
    "Lanza papa bomba",
    "Ataca al tombo",
    "No te dejes encerrar",
    "Muevete, que vienen mas",
    "Recoge las monedas",
    "Abrete paso",
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Outcome {
    Running,
    Cleared,
    Defeat,
}

pub struct Monster {
    pub id: usize,
    pub pos: Cell,
    pub previous: Cell,
    pub hp: i32,
    pub max_hp: i32,
    pub cooldown: f32,
    pub animation: f32,
    pub direction: usize,
    pub captain: bool,
}

struct FloatText {
    pos: Vec2,
    label: String,
    color: Color,
    life: f32,
}

pub struct Game {
    pub level_index: usize,
    pub level: Level,
    pub character: usize,
    pub pos: Cell,
    previous: Cell,
    animation: f32,
    direction: usize,
    pub hp: i32,
    pub score: i32,
    pub monsters: Vec<Monster>,
    path: Vec<Cell>,
    target: Option<usize>,
    move_cd: f32,
    attack_cd: f32,
    hurt: f32,
    elapsed: f32,
    texts: Vec<FloatText>,
    pub notice: f32,
    announcement_cd: f32,
    announcement_life: f32,
    announcement_index: usize,
}

impl Game {
    pub fn new(character: usize) -> Self {
        let mut game = Self {
            level_index: 0,
            level: level(0),
            character,
            pos: START,
            previous: START,
            animation: 1.,
            direction: 4,
            hp: 100,
            score: 0,
            monsters: vec![],
            path: vec![],
            target: None,
            move_cd: 0.,
            attack_cd: 0.,
            hurt: 0.,
            elapsed: 0.,
            texts: vec![],
            notice: 3.,
            announcement_cd: 3.,
            announcement_life: 0.,
            announcement_index: ANNOUNCEMENTS.len() - 1,
        };
        game.spawn_enemies();
        game
    }

    fn spawn_enemies(&mut self) {
        self.monsters = self
            .level
            .enemies
            .iter()
            .enumerate()
            .map(|(i, &pos)| {
                let captain =
                    self.level_index == LEVEL_COUNT - 1 && i == self.level.enemies.len() - 1;
                let hp = if captain {
                    100
                } else {
                    30 + self.level_index as i32 * 5
                };
                Monster {
                    id: i,
                    pos,
                    previous: pos,
                    hp,
                    max_hp: hp,
                    cooldown: 1.2 + i as f32 * 0.18,
                    animation: 1.,
                    direction: 4,
                    captain,
                }
            })
            .collect();
    }

    pub fn advance(&mut self) {
        assert!(self.level_index + 1 < LEVEL_COUNT);
        self.level_index += 1;
        self.level = level(self.level_index);
        self.hp = (self.hp + 25).min(100);
        self.score += 250;
        self.pos = START;
        self.previous = START;
        self.animation = 1.;
        self.path.clear();
        self.texts.clear();
        self.target = None;
        self.move_cd = 0.;
        self.attack_cd = 0.;
        self.hurt = 0.;
        self.notice = 3.;
        self.announcement_cd = 3.;
        self.announcement_life = 0.;
        self.spawn_enemies();
    }

    fn update_announcements(&mut self, dt: f32) {
        self.announcement_life = (self.announcement_life - dt).max(0.);
        if self.hp <= 0 || self.monsters.is_empty() {
            self.announcement_life = 0.;
            return;
        }
        if self.notice > 0. {
            return;
        }
        self.announcement_cd -= dt;
        if self.announcement_cd <= 0. {
            self.announcement_index = (self.announcement_index + 1) % ANNOUNCEMENTS.len();
            self.announcement_life = ANNOUNCEMENT_DURATION;
            self.announcement_cd = ANNOUNCEMENT_INTERVAL;
        }
    }

    pub fn outcome(&self) -> Outcome {
        if self.hp <= 0 {
            Outcome::Defeat
        } else if self.monsters.is_empty() && self.pos == EXIT {
            Outcome::Cleared
        } else {
            Outcome::Running
        }
    }

    fn message(&mut self, pos: Cell, label: String, color: Color) {
        self.texts.push(FloatText {
            pos: iso(pos, CAM) - vec2(0., 28.),
            label,
            color,
            life: 0.9,
        });
    }

    fn collect(&mut self) {
        if let Some(i) = self.level.coins.iter().position(|&p| p == self.pos) {
            self.level.coins.remove(i);
            self.score += 100;
            self.message(self.pos, "+100".into(), LIME);
        }
    }

    fn attack(&mut self, index: usize) {
        self.attack_cd = 0.28;
        let pos = self.monsters[index].pos;
        self.direction = facing(self.pos, pos);
        self.monsters[index].hp -= 15;
        self.message(pos, "-15".into(), INK);
        if self.monsters[index].hp <= 0 {
            let was_captain = self.monsters[index].captain;
            self.monsters.remove(index);
            self.score += if was_captain { 300 } else { 50 };
            if self.monsters.is_empty() {
                self.notice = 4.;
            }
        }
    }

    pub fn update(&mut self, dt: f32, mouse: Vec2) -> Outcome {
        let dt = dt.min(0.05);
        self.elapsed += dt;
        self.notice -= dt;
        self.move_cd -= dt;
        self.attack_cd -= dt;
        self.hurt -= dt;
        self.animation = (self.animation + dt / 0.13).min(1.);
        self.texts.retain_mut(|t| {
            t.life -= dt;
            t.pos.y -= dt * 16.;
            t.life > 0.
        });
        if is_mouse_button_pressed(MouseButton::Left) && mouse.y > 65. && mouse.y < 376. {
            let clicked_enemy = self
                .monsters
                .iter()
                .find(|m| {
                    let p = iso(m.pos, CAM);
                    Rect::new(p.x - 9., p.y - 21., 18., 24.).contains(mouse)
                })
                .map(|m| m.pos);
            if let Some(cell) = clicked_enemy.or_else(|| pick(mouse, CAM))
                && self.level.map[cell.1][cell.0] == Tile::Floor
            {
                self.target = self.monsters.iter().find(|m| m.pos == cell).map(|m| m.id);
                self.path = bfs(&self.level.map, self.pos, cell);
            }
        }
        let keyboard = if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            Some((0, -1))
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            Some((0, 1))
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            Some((-1, 0))
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            Some((1, 0))
        } else {
            None
        };
        if let Some((dx, dy)) = keyboard {
            self.target = None;
            self.path.clear();
            if let Some(cell) = neighbor(self.pos, dx, dy)
                && self.level.map[cell.1][cell.0] == Tile::Floor
            {
                self.path.push(cell);
            }
        }
        if let Some(id) = self.target {
            if let Some(i) = self.monsters.iter().position(|m| m.id == id) {
                let pos = self.monsters[i].pos;
                if dist(self.pos, pos) == 1 {
                    self.path.clear();
                    if self.attack_cd <= 0. {
                        self.attack(i);
                    }
                } else {
                    self.path = bfs(&self.level.map, self.pos, pos);
                }
            } else {
                self.target = None;
                self.path.clear();
            }
        }
        if is_key_down(KeyCode::Space)
            && self.attack_cd <= 0.
            && let Some(i) = self
                .monsters
                .iter()
                .position(|m| dist(self.pos, m.pos) == 1)
        {
            self.attack(i);
        }
        if self.move_cd <= 0. && !self.path.is_empty() {
            let next = self.path[0];
            if let Some(i) = self.monsters.iter().position(|m| m.pos == next) {
                if self.attack_cd <= 0. {
                    self.attack(i);
                }
            } else {
                self.path.remove(0);
                self.previous = self.pos;
                self.direction = facing(self.pos, next);
                self.pos = next;
                self.animation = 0.;
                self.move_cd = 0.15;
                self.collect();
            }
        }
        for i in 0..self.monsters.len() {
            self.monsters[i].animation = (self.monsters[i].animation + dt / 0.18).min(1.);
            self.monsters[i].cooldown -= dt;
            if self.monsters[i].cooldown > 0. {
                continue;
            }
            self.monsters[i].cooldown = 1.1 - self.level_index as f32 * 0.09;
            let pos = self.monsters[i].pos;
            self.monsters[i].direction = facing(pos, self.pos);
            if dist(pos, self.pos) == 1 {
                let damage = if self.monsters[i].captain {
                    9
                } else {
                    4 + self.level_index as i32 / 2
                };
                self.hp = (self.hp - damage).max(0);
                self.hurt = 0.17;
                self.message(self.pos, format!("-{damage}"), CORAL);
            } else {
                let path = bfs(&self.level.map, pos, self.pos);
                if let Some(&next) = path.first()
                    && next != self.pos
                    && !self.monsters.iter().any(|m| m.pos == next)
                {
                    self.monsters[i].previous = pos;
                    self.monsters[i].pos = next;
                    self.monsters[i].animation = 0.;
                }
            }
        }
        self.update_announcements(dt);
        self.outcome()
    }

    pub fn draw(&self, art: &Art, mouse: Vec2, time: f32) {
        backdrop(time);
        let hover = pick(mouse, CAM);
        for y in 0..MAP {
            for x in 0..MAP {
                let pos = iso((x, y), CAM);
                art.tile(pos, self.level.theme, false, 1.);
                if self.level.map[y][x] == Tile::Floor {
                    if self.path.contains(&(x, y)) {
                        diamond(pos, 3., 1.5, PURPLE);
                    }
                    if hover == Some((x, y)) && mouse.y > 65. {
                        diamond(pos, 17., 8., Color::new(0.9, 1., 0.8, 0.18));
                    }
                    if (x, y) == EXIT {
                        let color = if self.monsters.is_empty() {
                            LIME
                        } else {
                            PURPLE
                        };
                        diamond(
                            pos,
                            15.,
                            7.,
                            Color {
                                a: 0.3 + (time * 3.).sin() * 0.1,
                                ..color
                            },
                        );
                        diamond(pos, 9., 4., color);
                        diamond(pos, 6., 2., PANEL);
                        if self.monsters.is_empty() {
                            text(
                                "SALIDA",
                                pos.x - 17.,
                                pos.y - 22. - (time * 3.).sin() * 2.,
                                1.,
                                LIME,
                            );
                        }
                    }
                    if self.level.coins.contains(&(x, y)) {
                        let lift = (time * 3. + x as f32).sin() * 1.5;
                        diamond(pos - vec2(0., 5. + lift), 3., 5., GOLD);
                        draw_rectangle(pos.x - 1., pos.y - 8. - lift, 1., 5., INK);
                    }
                }
            }
        }
        // Draw walls and actors back to front, so foreground blocks occlude correctly.
        for depth in 0..MAP * 2 - 1 {
            for y in 0..MAP {
                if depth < y || depth - y >= MAP {
                    continue;
                }
                let x = depth - y;
                if self.level.map[y][x] == Tile::Wall {
                    art.tile(iso((x, y), CAM), self.level.theme, true, 1.);
                }
                if self.pos == (x, y) {
                    let p = iso(self.previous, CAM).lerp(iso(self.pos, CAM), self.animation);
                    if self.attack_cd > 0.18 {
                        diamond(p + vec2(0., 1.), 12., 6., Color::new(1., 1., 0.8, 0.35));
                    }
                    art.actor(
                        p,
                        self.character,
                        self.direction,
                        self.animation < 1.,
                        time,
                        1.,
                        if self.hurt > 0. { CORAL } else { WHITE },
                    );
                    draw_rectangle(
                        p.x - 1.,
                        p.y - 24.,
                        3.,
                        2.,
                        if self.character == 2 { CORAL } else { BLUE },
                    );
                }
                for m in &self.monsters {
                    if m.pos != (x, y) {
                        continue;
                    }
                    let p = iso(m.previous, CAM).lerp(iso(m.pos, CAM), m.animation);
                    let scale = if m.captain { 1.25 } else { 1. };
                    art.actor(p, 0, m.direction, m.animation < 1., time, scale, WHITE);
                    if m.hp < m.max_hp || m.captain || self.target == Some(m.id) {
                        draw_rectangle(p.x - 8., p.y - 26. * scale, 16., 3., BG);
                        draw_rectangle(
                            p.x - 8.,
                            p.y - 26. * scale,
                            16. * m.hp as f32 / m.max_hp as f32,
                            2.,
                            if m.captain { CORAL } else { LIME },
                        );
                    }
                    if m.captain {
                        text("!", p.x - 2., p.y - 42., 1., CORAL);
                    }
                }
            }
        }
        for t in &self.texts {
            centered(
                &t.label,
                t.pos.x,
                t.pos.y,
                1.,
                Color {
                    a: (t.life * 2.).min(1.),
                    ..t.color
                },
            );
        }
        draw_rectangle(0., 0., W, 62., BG);
        draw_line(16., 61., 624., 61., 1., EDGE);
        text("CRABLO", 17., 12., 2., LIME);
        text(
            &format!("{:02} / 05  {}", self.level_index + 1, self.level.name),
            122.,
            14.,
            1.,
            INK,
        );
        text(
            &format!("PATRULLAS {:02}", self.monsters.len()),
            122.,
            35.,
            1.,
            if self.monsters.is_empty() {
                LIME
            } else {
                MUTED
            },
        );
        text(&format!("PUNTOS {:05}", self.score), 350., 14., 1., GOLD);
        text("VIDA", 495., 14., 1., MUTED);
        draw_rectangle(495., 28., 100., 7., EDGE);
        draw_rectangle(
            495.,
            28.,
            self.hp.max(0) as f32,
            7.,
            if self.hp < 30 { CORAL } else { LIME },
        );
        text(&format!("{}", self.hp), 604., 28., 1., INK);
        for i in 0..LEVEL_COUNT {
            draw_rectangle(
                350. + i as f32 * 19.,
                34.,
                14.,
                4.,
                if i <= self.level_index { LIME } else { EDGE },
            );
        }
        draw_rectangle(0., 379., W, 21., BG);
        centered(
            if self.monsters.is_empty() {
                "SECTOR DESPEJADO - VE A LA SALIDA VERDE"
            } else {
                "CLIC: MOVER / ATACAR   WASD: MOVER   ESPACIO: GOLPE   ESC: PAUSA"
            },
            320.,
            386.,
            1.,
            if self.monsters.is_empty() {
                LIME
            } else {
                MUTED
            },
        );
        if self.notice > 0. {
            panel(Rect::new(165., 66., 310., 20.), EDGE);
            centered(
                if self.monsters.is_empty() {
                    "PATRULLAS FUERA. BUSCA LA SALIDA."
                } else {
                    self.level.subtitle
                },
                320.,
                73.,
                1.,
                LIME,
            );
        } else if self.announcement_life > 0. {
            let label = ANNOUNCEMENTS[self.announcement_index];
            let width = (text_width(label, 1.) + 28.).clamp(192., 310.);
            let x = (W - width) / 2.;
            panel(Rect::new(x, 66., width, 22.), PURPLE);
            centered(label, W / 2., 73., 1., INK);
            draw_rectangle(
                x + 4.,
                84.,
                (width - 8.) * self.announcement_life / ANNOUNCEMENT_DURATION,
                1.,
                PURPLE,
            );
        }
    }
}

fn facing(from: Cell, to: Cell) -> usize {
    match (to.0 as i32 - from.0 as i32, to.1 as i32 - from.1 as i32) {
        (dx, dy) if dx.abs() > dy.abs() => {
            if dx > 0 {
                3
            } else {
                7
            }
        }
        (_, dy) => {
            if dy > 0 {
                5
            } else {
                1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn announcements_rotate_with_gaps_and_stop_when_combat_ends() {
        let mut game = Game::new(1);
        game.update_announcements(3.);
        assert_eq!(game.announcement_life, 0., "The level intro takes priority");
        game.notice = 0.;
        game.update_announcements(3.);
        assert_eq!(ANNOUNCEMENTS[game.announcement_index], "Lanza papa bomba");
        assert_eq!(game.announcement_life, ANNOUNCEMENT_DURATION);
        game.update_announcements(ANNOUNCEMENT_DURATION);
        assert_eq!(game.announcement_life, 0.);
        game.update_announcements(ANNOUNCEMENT_INTERVAL - ANNOUNCEMENT_DURATION);
        assert_eq!(ANNOUNCEMENTS[game.announcement_index], "Ataca al tombo");
        game.monsters.clear();
        game.update_announcements(0.01);
        assert_eq!(game.announcement_life, 0.);
        game.advance();
        assert_eq!(game.announcement_life, 0.);
        game.notice = 0.;
        game.hp = 0;
        game.update_announcements(ANNOUNCEMENT_INTERVAL);
        assert_eq!(game.announcement_life, 0.);
    }

    #[test]
    fn campaign_requires_exit_and_preserves_choice_health_and_score() {
        let mut game = Game::new(2);
        for index in 0..LEVEL_COUNT {
            assert_eq!(game.level_index, index);
            assert_eq!(game.character, 2);
            game.monsters.clear();
            assert_eq!(game.outcome(), Outcome::Running);
            game.pos = EXIT;
            assert_eq!(game.outcome(), Outcome::Cleared);
            if index + 1 < LEVEL_COUNT {
                game.hp = 60;
                let score = game.score;
                game.advance();
                assert_eq!(game.hp, 85);
                assert_eq!(game.score, score + 250);
                assert!(!game.monsters.is_empty());
                assert_eq!(game.pos, START);
            }
        }
        assert_eq!(game.monsters.len(), 0);
        game.hp = 0;
        assert_eq!(game.outcome(), Outcome::Defeat);
    }
    #[test]
    fn coins_only_score_once_and_captain_has_extra_health() {
        let mut game = Game::new(1);
        game.pos = (3, 3);
        game.collect();
        game.collect();
        assert_eq!(game.score, 100);
        for _ in 1..LEVEL_COUNT {
            game.advance();
        }
        let captain = game.monsters.iter().find(|m| m.captain).unwrap();
        assert_eq!(captain.hp, 100);
        assert_eq!(game.character, 1);
    }
}
