//! What the screen says (`sbar.c`, `screen.c`, `console.c`), in letters drawn from lines rather
//! than the original's pictures: health, armour and ammunition, the weapons held, keys and
//! power-ups; messages in the middle and in the corner; the level's tally; the sight.

use game::Game;
use glam::Vec4;
use render::{Text, text_width};

/// `scr_centertime`: how long a message in the middle stays.
pub const CENTRE_SECONDS: f32 = 2.0;
/// `con_notifytime`: how long a line in the corner stays.
const NOTIFY_SECONDS: f32 = 3.0;
/// `scr_printspeed`: the finale's letters a second.
const PRINT_SPEED: f32 = 8.0;

const WHITE: Vec4 = Vec4::new(0.92, 0.9, 0.86, 1.0);
const DIM: Vec4 = Vec4::new(0.55, 0.53, 0.5, 0.8);
const GOLD: Vec4 = Vec4::new(1.0, 0.78, 0.35, 1.0);
const RED: Vec4 = Vec4::new(1.0, 0.3, 0.2, 1.0);

/// The texts of one frame, laid out on a screen `w × h` pixels.
pub struct Status<'a> {
    pub w: f32,
    pub h: f32,
    pub texts: &'a mut Vec<Text>,
}

impl Status<'_> {
    /// The screen's measure: 1 at 1080 lines, more on a taller screen.
    fn u(&self) -> f32 {
        self.h / 1080.0
    }

    fn put(&mut self, x: f32, y: f32, size: f32, colour: Vec4, text: impl Into<String>) {
        let text = text.into();
        // a shadow one stroke down and right, so letters read over any wall
        let s = size / 6.0 * 0.6;
        self.texts.push(Text { x: x + s, y: y + s, size, colour: Vec4::new(0.0, 0.0, 0.0, colour.w * 0.7), text: text.clone() });
        self.texts.push(Text { x, y, size, colour, text });
    }

    fn centred(&mut self, y: f32, size: f32, colour: Vec4, text: &str) {
        let x = (self.w - text_width(text, size)) * 0.5;
        self.put(x, y, size, colour, text);
    }

    /// The status: health and armour on the left, ammunition and the weapons held on the right,
    /// keys and power-ups between (`Sbar_Draw`).
    pub fn bar(&mut self, g: &Game) {
        let p = &g.ents[1].v;
        let u = self.u();
        let base = self.h - 36.0 * u;
        let big = 44.0 * u;
        let small = 13.0 * u;
        let health = p.health.max(0.0) as i32;
        self.put(40.0 * u, base, big, if health <= 25 { RED } else { WHITE }, health.to_string());
        self.put(40.0 * u, base + 22.0 * u, small, DIM, "HEALTH");
        if p.armorvalue > 0.0 {
            // the armour's colour by its kind: green (0.3), yellow (0.6), red (0.8)
            let c = if p.items & game::IT_ARMOR3 != 0 {
                Vec4::new(0.95, 0.35, 0.3, 1.0)
            } else if p.items & game::IT_ARMOR2 != 0 {
                Vec4::new(0.95, 0.85, 0.3, 1.0)
            } else {
                Vec4::new(0.45, 0.85, 0.4, 1.0)
            };
            self.put(220.0 * u, base, big, c, (p.armorvalue as i32).to_string());
            self.put(220.0 * u, base + 22.0 * u, small, DIM, "ARMOR");
        }
        // the ammunition of the weapon in hand
        let kind = if p.items & game::IT_SHELLS != 0 {
            "SHELLS"
        } else if p.items & game::IT_NAILS != 0 {
            "NAILS"
        } else if p.items & game::IT_ROCKETS != 0 {
            "ROCKETS"
        } else if p.items & game::IT_CELLS != 0 {
            "CELLS"
        } else {
            ""
        };
        if !kind.is_empty() {
            let ammo = (p.currentammo as i32).to_string();
            let x = self.w - 40.0 * u - text_width(&ammo, big);
            self.put(x, base, big, if p.currentammo < 10.0 { RED } else { WHITE }, ammo);
            let x = self.w - 40.0 * u - text_width(kind, small);
            self.put(x, base + 22.0 * u, small, DIM, kind);
        }
        // the weapons held, 1 to 8, the one in hand brighter (`Sbar_DrawInventory`)
        let weapons = [game::IT_AXE, game::IT_SHOTGUN, game::IT_SUPER_SHOTGUN, game::IT_NAILGUN, game::IT_SUPER_NAILGUN, game::IT_GRENADE_LAUNCHER, game::IT_ROCKET_LAUNCHER, game::IT_LIGHTNING];
        let step = 22.0 * u;
        let row = base - big - 14.0 * u;
        for (i, w) in weapons.iter().enumerate() {
            let x = self.w - 40.0 * u - (8 - i) as f32 * step;
            let c = if p.weapon == *w { GOLD } else if p.items & w != 0 { WHITE } else { Vec4::new(0.4, 0.38, 0.35, 0.5) };
            self.put(x, row, 16.0 * u, c, (i + 1).to_string());
        }
        // keys and power-ups, between, with the power-ups' seconds left
        let mut things: Vec<(String, Vec4)> = Vec::new();
        if p.items & game::IT_KEY1 != 0 {
            things.push(("SILVER KEY".into(), Vec4::new(0.8, 0.85, 0.95, 1.0)));
        }
        if p.items & game::IT_KEY2 != 0 {
            things.push(("GOLD KEY".into(), GOLD));
        }
        let left = |finished: f32| ((finished - g.time).max(0.0)).ceil() as i32;
        if p.items & game::IT_QUAD != 0 {
            things.push((format!("QUAD {}", left(p.super_damage_finished)), Vec4::new(0.4, 0.6, 1.0, 1.0)));
        }
        if p.items & game::IT_INVULNERABILITY != 0 {
            things.push((format!("PENTAGRAM {}", left(p.invincible_finished)), RED));
        }
        if p.items & game::IT_INVISIBILITY != 0 {
            things.push((format!("RING {}", left(p.invisible_finished)), WHITE));
        }
        if p.items & game::IT_SUIT != 0 {
            things.push((format!("SUIT {}", left(p.radsuit_finished)), Vec4::new(0.4, 0.9, 0.4, 1.0)));
        }
        let joined: Vec<String> = things.iter().map(|t| t.0.clone()).collect();
        let line = joined.join("   ");
        let mut x = (self.w - text_width(&line, small * 1.2)) * 0.5;
        for (t, c) in &things {
            self.put(x, base, small * 1.2, *c, t.clone());
            x += text_width(&format!("{t}   "), small * 1.2) + small * 1.2 / 6.0 * 2.0;
        }
    }

    /// The sight in the middle of the screen (`crosshair`).
    pub fn sight(&mut self) {
        let u = self.u();
        let size = 12.0 * u;
        let x = (self.w - text_width("+", size)) * 0.5;
        self.texts.push(Text { x, y: self.h * 0.5 + size * 0.5, size, colour: Vec4::new(1.0, 1.0, 1.0, 0.5), text: "+".into() });
    }

    /// A message in the middle, line by line, for its time (`SCR_DrawCenterString`).
    pub fn centre(&mut self, text: &str) {
        let u = self.u();
        let size = 22.0 * u;
        let lines: Vec<&str> = text.lines().collect();
        let mut y = self.h * 0.35 - (lines.len() as f32 - 1.0) * size * 0.9;
        for l in lines {
            self.centred(y, size, WHITE, l);
            y += size * 1.8;
        }
    }

    /// The last lines printed, in the corner, each for three seconds (`Con_DrawNotify`).
    pub fn notify(&mut self, lines: &[(String, f32)], now: f32) {
        let u = self.u();
        let size = 15.0 * u;
        let shown: Vec<&(String, f32)> = lines.iter().filter(|(_, t)| now - t < NOTIFY_SECONDS).collect();
        let mut y = 40.0 * u;
        for (l, _) in shown.iter().rev().take(4).rev() {
            self.put(24.0 * u, y, size, WHITE, l.trim_end().to_string());
            y += size * 1.8;
        }
    }

    /// The level's tally at its end (`Sbar_IntermissionOverlay`): its time, secrets and kills.
    pub fn tally(&mut self, g: &Game, seconds: f32) {
        let u = self.u();
        let big = 30.0 * u;
        let mut y = self.h * 0.3;
        self.centred(y, big * 1.3, GOLD, "COMPLETED");
        y += big * 3.0;
        let t = seconds.max(0.0) as i32;
        self.centred(y, big, WHITE, &format!("TIME    {}:{:02}", t / 60, t % 60));
        y += big * 2.0;
        self.centred(y, big, WHITE, &format!("SECRETS {:>3}/{:<3}", g.found_secrets as i32, g.total_secrets as i32));
        y += big * 2.0;
        self.centred(y, big, WHITE, &format!("KILLS   {:>3}/{:<3}", g.killed_monsters as i32, g.total_monsters as i32));
    }

    /// The episode's end, typed out at eight letters a second (`SCR_DrawCenterString` in finale).
    pub fn finale(&mut self, text: &str, since: f32) {
        let shown: String = text.chars().take((since * PRINT_SPEED) as usize).collect();
        self.centre(&shown);
    }
}
