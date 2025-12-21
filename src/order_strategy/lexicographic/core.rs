use crate::{hypercube::Hypercube, order_strategy::OrderStrategy};

pub struct LexicographicOrder {
    current_index: usize,
    stop_value: usize, // exclusive
    num_vars: usize,
}

impl OrderStrategy for LexicographicOrder {
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
            self.current_index += 1;
            this_index
        } else {
            None
        }
    }

    fn num_vars(&self) -> usize {
        self.num_vars
    }
}

impl Iterator for LexicographicOrder {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_index()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexicographic_order_3_vars() {
        let num_vars = 3;
        let mut order = LexicographicOrder::new(num_vars);
        
        // Expected sequence: 000, 001, 010, 011, 100, 101, 110, 111
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut actual = Vec::new();

        while let Some(index) = order.next() {
            actual.push(index);
        }

        assert_eq!(actual, expected, "Lexicographic order should be a simple range 0..2^n");
    }

    #[test]
    fn test_lexicographic_completeness() {
        let num_vars = 8; // 256 elements
        let expected_count = 1 << num_vars;
        let order = LexicographicOrder::new(num_vars);
        
        let results: Vec<usize> = order.collect();

        assert_eq!(results.len(), expected_count);
        
        // Verify it is strictly increasing by 1
        for i in 0..expected_count {
            assert_eq!(results[i], i);
        }
    }

    #[test]
    fn test_lexicographic_zero_vars() {
        let mut order = LexicographicOrder::new(0);
        // 2^0 = 1 element (0)
        assert_eq!(order.next(), Some(0));
        assert_eq!(order.next(), None);
    }

    #[test]
    fn test_lexicographic_one_var() {
        let mut order = LexicographicOrder::new(1);
        assert_eq!(order.next(), Some(0));
        assert_eq!(order.next(), Some(1));
        assert_eq!(order.next(), None);
    }
}