#![allow(dead_code)]

// INJECT: src/template/math/modint.rs

// --- SNAP START ---

pub struct Combination<const M: u64> {
    fact: Vec<ModInt<M>>,
    inv_fact: Vec<ModInt<M>>,
}

impl<const M: u64> Combination<M> {
    pub fn new(max_n: usize) -> Self {
        let mut fact = vec![ModInt::new(1); max_n + 1];
        let mut inv_fact = vec![ModInt::new(1); max_n + 1];

        for i in 1..=max_n {
            fact[i] = fact[i - 1] * (i as i64);
        }

        inv_fact[max_n] = fact[max_n].inv();
        for i in (1..=max_n).rev() {
            inv_fact[i - 1] = inv_fact[i] * (i as i64);
        }

        Combination { fact, inv_fact }
    }

    pub fn ncr(
        &self,
        n: usize,
        r: usize,
    ) -> ModInt<M> {
        if r > n {
            return ModInt::new(0);
        }
        self.fact[n] * self.inv_fact[r] * self.inv_fact[n - r]
    }

    pub fn npr(
        &self,
        n: usize,
        r: usize,
    ) -> ModInt<M> {
        if r > n {
            return ModInt::new(0);
        }
        self.fact[n] * self.inv_fact[n - r]
    }

    pub fn nhr(
        &self,
        n: usize,
        r: usize,
    ) -> ModInt<M> {
        if n == 0 && r == 0 {
            return ModInt::new(1);
        }
        self.ncr(n + r - 1, r)
    }

    pub fn fact(
        &self,
        n: usize,
    ) -> ModInt<M> {
        self.fact[n]
    }
}

// --- SNAP END ---
