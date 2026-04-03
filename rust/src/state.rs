use std::collections::BTreeMap;

/// Camel colors
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Camel {
    WHITE,
    YELLOW,
    ORANGE,
    GREEN,
    BLUE,
}

/// Game state
#[derive(Clone, Debug)]
pub struct State {
    /// Mapping from integer keys to lists of camels.
    pub data: BTreeMap<i32, Vec<Camel>>,
}

impl State {
    /// Create a new State from the provided map
    pub fn new(data: BTreeMap<i32, Vec<Camel>>) -> Self {
        State { data }
    }

    /// Move `camel` forward by `steps` (dice result).
    /// Moves `camel` and all following camels within the same field to `field + steps`.
    ///
    /// Panics if:
    /// - `steps < 1`
    /// - `camel` is not found in `data`
    /// - `camel` appears more than once in `data`
    pub fn move_camel(&self, camel: Camel, steps: i32) -> Self {
        if steps < 1 {
            panic!("steps must be >= 1, got {}", steps);
        }

        // Find the unique occurrence of camel: (field, position)
        let mut found: Option<(i32, usize)> = None;
        for (k, v) in &self.data {
            for (i, &c) in v.iter().enumerate() {
                if c == camel {
                    if found.is_some() {
                        panic!("camel {:?} appears multiple times in data", camel);
                    }
                    found = Some((*k, i));
                }
            }
        }

        let (field, position) = match found {
            Some(fp) => fp,
            None => panic!("camel {:?} not found in state.data", camel),
        };

        let new_field = field + steps;
        if new_field == field {
            // no-op
            return self.clone();
        }

        // Clone map and perform the mutation on the clone
        let mut map = self.data.clone();

        // Remove the source vector so we can mutate the map without overlapping borrows
        let mut src_vec = map
            .remove(&field)
            .unwrap_or_else(|| panic!("internal error: expected key {} present", field));

        // split_off returns the tail starting at `position`
        let tail = src_vec.split_off(position);

        // if there is a non-empty prefix, put it back under the original field
        if !src_vec.is_empty() {
            map.insert(field, src_vec);
        }

        // append the tail to destination vector (create if necessary)
        let dest_vec = map.entry(new_field).or_default();
        dest_vec.extend(tail);

        State { data: map }
    }

    /// Return all camels flattened in order by the map's key.
    /// This mirrors: [c for _, y in sorted(self.data.items(), key=lambda s: s[0]) for c in y][::1]
    pub fn order(&self) -> Vec<Camel> {
        self.data
            .iter()
            .flat_map(|(_, v)| v.iter().copied())
            .collect()
    }
}
