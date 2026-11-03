//! Rexy Sorter - entry point. Semua fungsi export extern "C" untuk Horizon.
mod config;
mod gui;
mod manifest;
mod profile;
mod sorter;
mod state;

use config::Config;
use gui::Button;
use sorter::Slots;
use state::{cstr, persist, to_c, with, State, STATE};
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub extern "C" fn hn_mod_init() -> c_int {
    let cfg = Config::load();
    let _ = manifest::ensure_written();
    let _ = cfg.save();
    let mut button = Button::new();
    button.set_mode(cfg.mode);
    if let Ok(mut g) = STATE.lock() {
        *g = Some(State { cfg, world: None, button });
        return 0;
    }
    -1
}

/// Dipanggil saat world dimuat: load profile per world (kalau ada).
#[no_mangle]
pub unsafe extern "C" fn hn_mod_on_world(name: *const c_char) {
    let Some(w) = cstr(name) else { return };
    with(|s| {
        if let Some(p) = profile::load(&w) {
            p.apply(&mut s.cfg);
            s.button.set_mode(s.cfg.mode);
        }
        s.world = Some(w);
    });
}

/// Dipanggil tiap inventory dibuka (player/chest/dll) untuk menaruh tombol.
#[no_mangle]
pub extern "C" fn hn_mod_on_inventory_open(screen_w: f32, screen_h: f32) {
    with(|s| s.button.place(screen_w, screen_h, s.cfg.show_button));
}

/// 1 = tap kena tombol Sort (host lalu panggil hn_mod_sort).
#[no_mangle]
pub extern "C" fn hn_mod_on_tap(x: f32, y: f32) -> c_int {
    with(|s| s.button.hit(x, y) as c_int).unwrap_or(0)
}

/// 1 = keybind (default R) ditekan.
#[no_mangle]
pub extern "C" fn hn_mod_on_key(keycode: c_int) -> c_int {
    with(|s| gui::key_matches(&s.cfg.keybind, keycode) as c_int).unwrap_or(0)
}

/// Label tombol saat ini (bebaskan dengan hn_mod_free).
#[no_mangle]
pub extern "C" fn hn_mod_button_label() -> *mut c_char {
    to_c(with(|s| s.button.label.clone()).unwrap_or_default())
}

/// Input JSON array slot [null | {"id","count","max"}], `start` = slot awal yang disort.
/// Output JSON array hasil sort, atau null kalau gagal.
#[no_mangle]
pub unsafe extern "C" fn hn_mod_sort(json: *const c_char, start: c_int) -> *mut c_char {
    let Some(txt) = cstr(json) else { return std::ptr::null_mut() };
    let Ok(mut slots) = serde_json::from_str::<Slots>(&txt) else { return std::ptr::null_mut() };
    with(|s| {
        sorter::sort(&mut slots, s.cfg.mode, &s.cfg.locked_slots, &s.cfg.custom_order, start.max(0) as usize)
    });
    to_c(serde_json::to_string(&slots).unwrap_or_default())
}

/// Auto refill slot hotbar yang habis. Null bila fitur off / tidak ada stok.
#[no_mangle]
pub unsafe extern "C" fn hn_mod_refill(json: *const c_char, slot: c_int, id: *const c_char) -> *mut c_char {
    let (Some(txt), Some(id)) = (cstr(json), cstr(id)) else { return std::ptr::null_mut() };
    let Ok(mut slots) = serde_json::from_str::<Slots>(&txt) else { return std::ptr::null_mut() };
    let on = with(|s| s.cfg.auto_refill).unwrap_or(false);
    if on && slot >= 0 && sorter::refill(&mut slots, slot as usize, &id) {
        return to_c(serde_json::to_string(&slots).unwrap_or_default());
    }
    std::ptr::null_mut()
}

/// Ganti mode (Alphabetical -> Category -> Custom). Return index mode baru.
#[no_mangle]
pub extern "C" fn hn_mod_cycle_mode() -> c_int {
    with(|s| {
        s.cfg.mode = s.cfg.mode.next();
        s.button.set_mode(s.cfg.mode);
        persist(s);
        s.cfg.mode.index()
    })
    .unwrap_or(-1)
}

/// Lock / unlock slot. Return 1 kalau sekarang terkunci.
#[no_mangle]
pub extern "C" fn hn_mod_toggle_lock(slot: c_int) -> c_int {
    if slot < 0 {
        return 0;
    }
    with(|s| {
        let locked = s.cfg.toggle_lock(slot as usize);
        persist(s);
        locked as c_int
    })
    .unwrap_or(0)
}

/// Toggle auto refill. Return 1 kalau sekarang aktif.
#[no_mangle]
pub extern "C" fn hn_mod_toggle_refill() -> c_int {
    with(|s| {
        s.cfg.auto_refill = !s.cfg.auto_refill;
        persist(s);
        s.cfg.auto_refill as c_int
    })
    .unwrap_or(0)
}

/// Bebaskan string yang dikembalikan mod ini.
#[no_mangle]
pub unsafe extern "C" fn hn_mod_free(p: *mut c_char) {
    if !p.is_null() {
        drop(CString::from_raw(p));
    }
}
