use std::collections::HashMap;

/// Defines the rules used by a system
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

    /// Return the next generation of a symbol according to the rules
    fn get_future(&self, symbol: u8) -> Vec<u8> {
        if let Some(next_gen) = self.rules.get(&symbol) {
            next_gen.clone()
        } else {
            // If there are no rules for this symbol it stays the same
            vec![symbol]
        }
    }
}

/// We simulate the L-System one symbol at a time
/// ```text
/// axiom:             A
///                   / \
/// n=1:             A   B
///                 /|    \
/// n=2:           A B     A
///              / | |     | \
/// n=3:         A B A     A B
///            / | | | \   | \ \
/// n=4:       A B A A B   A B A
/// ```
/// This diagram inlustrates the evolution of an L-System (courtesy of wikipedia)
///
/// `generations` stores how many generations there are (4 in this case)
///
/// `indexes` has length of generations, and indicates what was the last symbol we reached during our calculations
///
/// `indexes[0]` will be at most 0, because we only have one symbol in the axiom
///
/// `indexes[1]` will be at most 1, bacuase we have two symbols in the first generation
///
/// `indexes[2]` will be at most 2, because we have three symbos in the second generation etc...
///
/// `iterate_next` recounstructs each time the L-System tree, incrementing the last indexes
/// to give us the next symbols in the last generation
/// `increment` increments the indexes, staring from the last generation, and is called at the end of `interate_next`

pub struct LSystem {
    generations: usize,
    // Length is always == generations
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

    /// Gets the last generation's symbols, as an iterator (not an std one)
    pub fn iterate_over(&mut self, rules: &SystemRules) -> Option<Vec<u8>> {
        let mut sys = self.axiom.clone();

        // We have simulated the entire generation
        if self.indexes[0] >= sys.len() {
            return None;
        }

        // How many symbols does each generation contain
        let mut lengths = vec![0; self.generations];
        for (n, len) in lengths.iter_mut().enumerate() {
            // Get the last index we arrived upon
            let index = self.indexes[n];
            // How the system evolves
            let future = rules.get_future(sys[index]);
            // Set the length of this generation
            *len = sys.len();
            // The next generation will be the one we do calculations on
            sys = future;
        }

        self.increment(&lens);
        Some(sys)
    }
}
