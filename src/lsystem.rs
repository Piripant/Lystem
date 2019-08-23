use std::collections::HashMap;

pub struct SystemRules {
    rules: HashMap<u8, Vec<u8>>,
}

impl SystemRules {
    pub fn new() -> SystemRules {
        SystemRules {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, original: u8, transformation: Vec<u8>) {
        self.rules.insert(original, transformation);
    }

    // returns the next evolution of a symbol
    // according to the rules
    fn get_future(&self, symbol: u8) -> Vec<u8> {
        if let Some(rule) = self.rules.get(&symbol) {
            rule.clone()
        } else {
            vec![symbol]
        }
    }
}

pub struct LSystem {
    generations: usize,
    indexes: Vec<usize>,
    axiom: Vec<u8>,
}

impl LSystem {
    pub fn new(axiom: Vec<u8>, generations: usize) -> LSystem {
        assert!(generations > 0);
        LSystem {
            generations,
            indexes: vec![0; generations],
            axiom,
        }
    }

    fn increment(&mut self, lengths: &[usize]) {
        self.indexes[self.generations - 1] += 1;

        for i in (0..self.generations).rev() {
            if i > 0 && self.indexes[i] >= lengths[i] {
                self.indexes[i] = 0;
                self.indexes[i - 1] += 1;
            }
        }
    }

    pub fn iterate_over(&mut self, rules: &SystemRules) -> Option<Vec<u8>> {
        let mut sys = self.axiom.clone();

        // All the calculations have been done
        if self.indexes[0] >= sys.len() {
            return None;
        }

        let mut lens = vec![0; self.generations];
        for (n, len) in lens.iter_mut().enumerate() {
            let index = self.indexes[n];
            let future = rules.get_future(sys[index]);
            *len = sys.len();
            sys = future;
        }

        self.increment(&lens);

        Some(sys)
    }
}
