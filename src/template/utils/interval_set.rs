#![allow(dead_code)]
use std::collections::BTreeMap;

// --- SNAP START ---

#[derive(Debug, Clone)]
pub struct RangeSet {
    map: BTreeMap<i64, i64>,
}

impl RangeSet {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    // Adds [l, r) and returns the increased length.
    pub fn insert(
        &mut self,
        mut l: i64,
        mut r: i64,
    ) -> i64 {
        if l >= r {
            return 0;
        }
        let mut added = r - l;

        if let Some((&l0, &r0)) = self.map.range(..=l).next_back() {
            if r0 >= l {
                l = l0;
                r = r.max(r0);
                added -= r0 - l0;
                self.map.remove(&l0);
            }
        }

        let mut to_remove = Vec::new();
        for (&l1, &r1) in self.map.range(l..) {
            if l1 > r {
                break;
            }
            r = r.max(r1);
            added -= r1 - l1;
            to_remove.push(l1);
        }

        for key in to_remove {
            self.map.remove(&key);
        }

        self.map.insert(l, r);
        added
    }

    // Removes [l, r) and returns the decreased length.
    pub fn erase(
        &mut self,
        l: i64,
        r: i64,
    ) -> i64 {
        if l >= r {
            return 0;
        }
        let mut removed = 0;

        if let Some((&l0, &r0)) = self.map.range(..=l).next_back() {
            if r0 > l {
                removed += r0.min(r) - l;
                self.map.remove(&l0);
                if l0 < l {
                    self.map.insert(l0, l);
                }
                if r0 > r {
                    self.map.insert(r, r0);
                }
            }
        }

        let mut to_remove = Vec::new();
        let mut to_insert = Vec::new();
        for (&l1, &r1) in self.map.range(l..) {
            if l1 >= r {
                break;
            }
            to_remove.push(l1);
            if r1 <= r {
                removed += r1 - l1;
            } else {
                removed += r - l1;
                to_insert.push((r, r1));
            }
        }

        for key in to_remove {
            self.map.remove(&key);
        }
        for (nl, nr) in to_insert {
            self.map.insert(nl, nr);
        }

        removed
    }

    // Returns the range [l, r) containing x.
    pub fn get(
        &self,
        x: i64,
    ) -> Option<(i64, i64)> {
        if let Some((&l, &r)) = self.map.range(..=x).next_back() {
            if r > x {
                return Some((l, r));
            }
        }
        None
    }

    pub fn contains(
        &self,
        x: i64,
    ) -> bool {
        self.get(x).is_some()
    }

    // Returns the minimum excluded value >= x.
    pub fn mex(
        &self,
        x: i64,
    ) -> i64 {
        if let Some((_, r)) = self.get(x) {
            r
        } else {
            x
        }
    }
}
