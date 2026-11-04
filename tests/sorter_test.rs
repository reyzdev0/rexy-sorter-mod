use rexy_sorter::sorter::*;

fn it(id: &str, c: u32) -> Option<Item> {
    Some(Item { id: id.into(), count: c, max: 64 })
}

#[test]
fn sorts_merges_and_respects_lock() {
    let mut s = vec![it("dirt", 40), it("apple", 5), None, it("dirt", 40), it("bread", 1)];
    sort(&mut s, SortMode::Alphabetical, &[1], &[], 0);
    assert_eq!(s[1], it("apple", 5));
    assert_eq!(s[0].as_ref().unwrap().id, "bread");
    assert_eq!(s[2], it("dirt", 64));
}

#[test]
fn category_puts_tools_before_blocks() {
    let mut s = vec![it("dirt", 1), it("iron_pickaxe", 1)];
    sort(&mut s, SortMode::Category, &[], &[], 0);
    assert_eq!(s[0].as_ref().unwrap().id, "iron_pickaxe");
}

#[test]
fn refill_pulls_from_main_inventory() {
    let mut s: Slots = vec![None; 12];
    s[11] = it("bread", 3);
    assert!(refill(&mut s, 2, "bread"));
    assert_eq!(s[2], it("bread", 3));
    assert!(s[11].is_none());
}
