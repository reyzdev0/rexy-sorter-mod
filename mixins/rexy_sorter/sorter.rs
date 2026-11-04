// Pure sort logic: Alphabetical, Category, Custom, stack merge, auto refill.
// Free of the game, so it is unit tested on its own.
use std::cmp::Ordering;

pub const HOTBAR: usize = 9;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SortMode {
    #[default]
    Alphabetical,
    Category,
    Custom,
}

impl SortMode {
    pub fn next(self) -> Self {
        match self {
            SortMode::Alphabetical => SortMode::Category,
            SortMode::Category => SortMode::Custom,
            SortMode::Custom => SortMode::Alphabetical,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            SortMode::Alphabetical => "Alphabetical",
            SortMode::Category => "Category",
            SortMode::Custom => "Custom",
        }
    }
    pub fn from_name(s: &str) -> Self {
        match s {
            "Category" => SortMode::Category,
            "Custom" => SortMode::Custom,
            _ => SortMode::Alphabetical,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub id: String,
    pub count: u32,
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

/// Sort slots from `start` (HOTBAR = skip the hotbar). Locked slots stay put.
pub fn sort(slots: &mut Slots, mode: SortMode, locked: &[usize], custom: &[String], start: usize) {
    let free: Vec<usize> = (start..slots.len()).filter(|i| !locked.contains(i)).collect();
    let taken: Vec<Item> = free.iter().filter_map(|&i| slots[i].take()).collect();
    let mut items = merge(taken);
    items.sort_by(|a, b| cmp(a, b, mode, custom));
    for (n, &i) in free.iter().enumerate() {
        slots[i] = items.get(n).cloned();
    }
}

/// Auto refill: an empty hotbar slot takes the same item from the main inventory.
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
