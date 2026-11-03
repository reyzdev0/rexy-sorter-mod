//! Tombol [Rexy Sort] di inventory + keybind
use crate::config::SortMode;

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub visible: bool,
    pub label: String,
}

impl Button {
    pub fn new() -> Self {
        Button { x: 0.0, y: 0.0, w: 0.0, h: 0.0, visible: false, label: "[Rexy Sort]".into() }
    }

    /// Posisi relatif ke layar; dipanggil tiap inventory dibuka (inventory biasa, chest, dll).
    pub fn place(&mut self, screen_w: f32, screen_h: f32, show: bool) {
        self.w = screen_w * 0.14;
        self.h = screen_h * 0.07;
        self.x = screen_w * 0.72;
        self.y = screen_h * 0.10;
        self.visible = show;
    }

    pub fn hit(&self, px: f32, py: f32) -> bool {
        self.visible && px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }

    pub fn set_mode(&mut self, mode: SortMode) {
        self.label = format!("[Rexy Sort: {}]", mode.name());
    }
}

/// Cocokkan keycode Android (KEYCODE_A=29) atau GLFW (A=65) dengan keybind huruf, default "R".
pub fn key_matches(keybind: &str, keycode: i32) -> bool {
    match keybind.trim().to_ascii_uppercase().chars().next() {
        Some(c) if c.is_ascii_uppercase() => {
            let off = c as i32 - 'A' as i32;
            keycode == 29 + off || keycode == 65 + off
        }
        _ => false,
    }
}
