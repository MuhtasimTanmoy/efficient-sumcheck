use crate::{hypercube::Hypercube, order_strategy::OrderStrategy};

pub struct GraycodeOrder {
    current_index: usize,
    stop_value: usize, // exclusive
    num_vars: usize,
}

impl GraycodeOrder {
    pub fn next_gray_code(value: usize) -> usize {
        let mask = match value.count_ones() & 1 == 0 {
            true => 1,
            false => 1 << (value.trailing_zeros() + 1),
        };
        value ^ mask
    }
}

impl OrderStrategy for GraycodeOrder {
    fn new(num_vars: usize) -> Self {
        Self {
            current_index: 0,
            stop_value: Hypercube::<Self>::stop_value(num_vars), // exclusive
            num_vars,
        }
    }

    fn next_index(&mut self) -> Option<usize> {
        if self.current_index < self.stop_value {
            let this_index = Some(self.current_index);
            self.current_index = GraycodeOrder::next_gray_code(self.current_index);
            this_index
        } else {
            None
        }
    }

    fn num_vars(&self) -> usize {
        self.num_vars
    }
}

impl Iterator for GraycodeOrder {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_index()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_code_3_vars() {
        let num_vars = 3;
        let mut order = GraycodeOrder::new(num_vars);
        
        // Expected reflected Gray code sequence for 3 bits:
        // 000 (0), 001 (1), 011 (3), 010 (2), 110 (6), 111 (7), 101 (5), 100 (4)
        let expected = vec![0, 1, 3, 2, 6, 7, 5, 4];
        let mut actual = Vec::new();

        while let Some(index) = order.next() {
            actual.push(index);
        }

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gray_code_properties() {
        // Test for a larger hypercube to ensure properties hold
        let num_vars = 5;
        let expected_count = 1 << num_vars;
        let order = GraycodeOrder::new(num_vars);
        
        let results: Vec<usize> = order.collect();

        // 1. Check total count
        assert_eq!(results.len(), expected_count);

        // 2. Check uniqueness (using a bitset or sorting)
        let mut sorted_results = results.clone();
        sorted_results.sort();
        for i in 0..expected_count {
            assert_eq!(sorted_results[i], i, "Missing value {} in sequence", i);
        }

        // 3. Check Hamming Distance (exactly one bit flip per step)
        for i in 0..results.len() - 1 {
            let diff = results[i] ^ results[i + 1];
            assert!(
                diff.is_power_of_two(),
                "Multiple bits flipped between {} and {}: XOR result is {}",
                results[i], results[i+1], diff
            );
        }
    }

    #[test]
    fn test_empty_hypercube() {
        let mut order = GraycodeOrder::new(0);
        // 2^0 = 1 element (the value 0)
        assert_eq!(order.next(), Some(0));
        assert_eq!(order.next(), None);
    }
}
