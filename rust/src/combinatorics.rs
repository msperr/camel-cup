use std::vec::Vec;

/// Permutations iterator using Heap's algorithm.
/// Consumes an iterable (collected into Vec<T>) and yields Vec<T> permutations.
/// T must be Clone.
pub struct Permutations<T> {
    a: Vec<T>,
    c: Vec<usize>,
    i: usize,
    first: bool,
}

impl<T: Clone> Permutations<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let a: Vec<T> = iter.into_iter().collect();
        if a.is_empty() {
            panic!("Permutations::new requires a non-empty input iterator");
        }
        let n = a.len();
        let c = vec![0; n];
        Permutations {
            a,
            c,
            i: 0,
            first: true,
        }
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.a.len();
        if self.first {
            self.first = false;
            return Some(self.a.clone());
        }

        while self.i < n {
            if self.c[self.i] < self.i {
                let swap_idx = if self.i.is_multiple_of(2) {
                    0
                } else {
                    self.c[self.i]
                };
                self.a.swap(swap_idx, self.i);
                self.c[self.i] += 1;
                self.i = 0;
                return Some(self.a.clone());
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }
        None
    }
}

/// Product (cartesian power) iterator: yields all length-`repeat` vectors where each entry is drawn
/// from `choices` (with repetition). Equivalent to "choices^repeat".
pub struct Product<T> {
    choices: Vec<T>,
    indices: Vec<usize>,
    done: bool,
}

impl<T: Clone> Product<T> {
    pub fn new(choices: Vec<T>, repeat: usize) -> Self {
        if choices.is_empty() {
            panic!("Product::new requires a non-empty choices vector");
        }
        if repeat < 1 {
            panic!("Product::new requires repeat >= 1");
        }
        Product {
            choices,
            indices: vec![0; repeat],
            done: false,
        }
    }
}

impl<T: Clone> Iterator for Product<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Build current vector from indices
        let current: Vec<T> = self
            .indices
            .iter()
            .map(|&i| self.choices[i].clone())
            .collect();

        // advance indices lexicographically
        let mut pos = self.indices.len() - 1;
        loop {
            self.indices[pos] += 1;
            if self.indices[pos] >= self.choices.len() {
                self.indices[pos] = 0;
                if pos == 0 {
                    self.done = true;
                    break;
                } else {
                    pos -= 1;
                }
            } else {
                break;
            }
        }

        Some(current)
    }
}
