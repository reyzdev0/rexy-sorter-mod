//! Logika sort: Alphabetical, Category, Custom + merge stack + auto refill
use crate::config::SortMode;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub const HOTBAR: usize = 9;

fn d64() -> u32 {
    64
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub count: u32,
    #[serde(default = "d64")]
    pub max: u32,
}

pub type Slots = Vec<Option<Item>>;

pub fn category(id: &str) -> u8 {
    let n = id.to_lowercase();
    let has = |keys: &[&str]| keys.iter().any(|k| n.contains(k));
    if has(&["sword", "bow", "trident"]) {
        0
    } else if has(&["pickaxe", "axe", "shovel", "hoe", "shears"]) {
        1
    } else if has(&["helmet", "chestplate", "leggings", "boots", "shield"]) {
        2
    } else if has(&["log", "planks", "stone", "dirt", "sand", "glass", "brick", "wool"]) {
        3
    } else if has(&["apple", "bread", "beef", "pork", "chicken", "carrot", "potato"]) {
        4
    } else if has(&["ingot", "diamond", "emerald", "redstone", "coal", "dust"]) {
        5
    } else {
        6
    }
}

fn rank(i: &Item, custom: &[String]) -> usize {
    custom.iter().position(|c| *c == i.id).unwrap_or(usize::MAX)
}

fn cmp(a: &Item, b: &Item, mode: SortMode, custom: &[String]) -> Ordering {
    let by_id = a.id.cmp(&b.id).then(b.count.cmp(&a.count));
    match mode {
        SortMode::Alphabetical => by_id,
        SortMode::Category => category(&a.id).cmp(&category(&b.id)).then(by_id),
        SortMode::Custom => rank(a, custom).cmp(&rank(b, custom)).then(by_id),
    }
}

fn merge(items: Vec<Item>) -> Vec<Item> {
    let mut out: Vec<Item> = Vec::new();
    for it in items {
        let mut left = it.count;
        for o in out.iter_mut().filter(|o| o.id == it.id && o.count < o.max) {
            let add = left.min(o.max - o.count);
            o.count += add;
            left -= add;
            if left == 0 {
                break;
            }
        }
        while left > 0 {
            let n = left.min(it.max.max(1));
            out.push(Item { id: it.id.clone(), count: n, max: it.max });
            left -= n;
        }
    }
    out
}

/// Sort slot mulai dari `start` (9 = lewati hotbar). Slot terkunci tidak disentuh.
pub fn sort(slots: &mut Slots, mode: SortMode, locked: &[usize], custom: &[String], start: usize) {
    let free: Vec<usize> = (start..slots.len()).filter(|i| !locked.contains(i)).collect();
    let taken: Vec<Item> = free.iter().filter_map(|&i| slots[i].take()).collect();
    let mut items = merge(taken);
    items.sort_by(|a, b| cmp(a, b, mode, custom));
    for (n, &i) in free.iter().enumerate() {
        slots[i] = items.get(n).cloned();
    }
}

/// Auto refill: isi slot hotbar kosong dengan item sama dari inventory utama.
pub fn refill(slots: &mut Slots, slot: usize, id: &str) -> bool {
    if slot >= HOTBAR || slots.get(slot).map_or(true, |s| s.is_some()) {
        return false;
    }
    let src = (HOTBAR..slots.len()).find(|&i| slots[i].as_ref().is_some_and(|s| s.id == id));
    match src {
        Some(i) => {
            slots[slot] = slots[i].take();
            true
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn it(id: &str, c: u32) -> Option<Item> {
        Some(Item { id: id.into(), count: c, max: 64 })
    }
    #[test]
    fn sorts_merges_and_respects_lock() {
        let mut s = vec![it("dirt", 40), it("apple", 5), None, it("dirt", 40), it("bread", 1)];
        sort(&mut s, SortMode::Alphabetical, &[1], &[], 0);
        assert_eq!(s[1], it("apple", 5));
        assert_eq!(s[0].as_ref().unwrap().id, "bread");
    }
}
