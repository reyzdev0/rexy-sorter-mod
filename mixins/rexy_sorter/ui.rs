// Options > Mods: this mod's row and its settings page.
use super::rexy_sorter_config as config;
use crate::game::native_hooks::NativeMod;

const ID: &str = "rexy_sorter";

pub fn register(mods: &mut Vec<NativeMod>) {
    let c = config::get();
    mods.push(NativeMod {
        id: ID.to_owned(),
        title: "Rexy Sorter 1.0.0".to_owned(),
        description: format!(
            "Inventory sorter (IPN-like): modes Alphabetical, Category and Custom, \
             key {}, lock slot, auto refill. Settings live in \
             horizon-mods/.horizon-mod-data/rexy_sorter/config.json.",
            c.keybind
        ),
        keys: Vec::new(),
    });
}

fn on_off(v: bool) -> &'static str {
    if v { "on" } else { "off" }
}

pub fn settings(mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != ID {
        return;
    }
    let c = config::get();
    buttons.push(format!("Sort mode: {} (click to change)", c.mode.name()));
    buttons.push(format!("Auto refill: {}", on_off(c.auto_refill)));
    buttons.push(format!("Sort button in inventory: {}", on_off(c.show_button)));
    buttons.push(format!("Locked slots: {} (edit config.json)", c.locked_slots.len()));
}

pub fn click(mod_id: &str, button: usize) {
    if mod_id != ID {
        return;
    }
    match button {
        0 => eprintln!("[rexy_sorter] mode: {}", config::cycle_mode().name()),
        1 => eprintln!("[rexy_sorter] auto refill: {}", on_off(config::toggle_refill())),
        2 => eprintln!("[rexy_sorter] button: {}", on_off(config::toggle_button())),
        _ => {}
    }
}
