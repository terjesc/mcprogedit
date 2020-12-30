//! Inventory

use crate::item::Item;
use crate::nbt_lookup::*;

use std::collections::HashMap;

// TODO / FUTURE WORK
// Maybe this is best as a trait?
// Then various implementations for the various inventories,
// as documented on https://wiki.vg/Inventory
//
// TODO / FUTURE WORK
// Special inventories can have special functions?
// e.g. furnace has "fuel" slot, "cooking" slot and "cooked" slot,
// and animals have "saddle" slot, "armor" slot, "chest" slots, etc.

#[derive(Clone, Debug, PartialEq)]
pub struct Inventory {
    slots: HashMap<i8, ItemStack>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    pub fn from_nbt_value_vec(list: &[nbt::Value]) -> Self {
        let mut slots = HashMap::new();
        for item in list {
            let slot = nbt_value_lookup_byte(&item, "Slot").unwrap();
            slots.insert(slot, ItemStack::from_nbt_value(&item));
        }
        Inventory { slots }
    }

    // TODO / FUTURE WORK
    // Interface:
    // Put stack in slot
    // Get stack from slot
    // Get capacity?
    // Get number of empty slots?
    // Put stack somewhere (if space)
    // Iterate over slots?
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ItemStack {
    item: Item,
    count: i8,
}

impl ItemStack {
    pub fn from_nbt_value(value: &nbt::Value) -> Self {
        let count = nbt_value_lookup_byte(&value, "Count").unwrap();
        let item = Item::from_nbt_value(&value);

        Self { item, count }
    }

    // TODO / FUTURE WORK
    // Interface:
    // Set item "fn with_item(item: Item)"?
    // Set count "fn with_count(count: i8)"?
    // Add items to stack (adds items if possible, returns any excess)
    // Remove items from stack (returns the items)
    // Split stack (returns the half taken off the stac)
    // Replace stack (returns the original stack)
}
