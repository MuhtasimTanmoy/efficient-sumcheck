use crate::{hypercube::Hypercube, order_strategy::OrderStrategy};

pub struct SignificantBitOrder {
    current_index: usize,
    stop_value: usize, // exclusive
    num_vars: usize,
}

// we're using the usize like a vec<bool>, so we can't just reverse the whole thing .reverse_bits()
impl SignificantBitOrder {
    pub fn next_value_in_msb_order(x: usize, n: u32) -> usize {
        let mut result = x;
        for i in (0..n).rev() {
            result ^= 1 << i;
            if result >> i == 1 {
                break;
            }
        }
        result
    }
}

impl OrderStrategy for SignificantBitOrder {
    fn new(num_vars: usize) -> Self {
        Self {
            current_index: 0,
            stop_value: Hypercube::<Self>::stop_value(num_vars), // exclusive
            num_vars,
        }
    }

    fn next_index(&mut self) -> Option<usize> {
        if self.current_index < self.stop_value {
            let old_index = self.current_index;
            self.current_index = SignificantBitOrder::next_value_in_msb_order(
                self.current_index,
                self.num_vars as u32,
            );
            if self.current_index == 0 {
                // if the sequence rounds back to 0, we need to stop
                self.current_index = self.stop_value;
            }
            Some(old_index)
        } else {
            None
        }
    }

    fn num_vars(&self) -> usize {
        self.num_vars
    }
}

impl Iterator for SignificantBitOrder {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_index()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msb_order_3_vars() {
        let num_vars = 3;
        let mut order = SignificantBitOrder::new(num_vars);
        
        // Standard Binary (LSB): 000, 001, 010, 011, 100, 101, 110, 111 (0,1,2,3,4,5,6,7)
        // MSB Order (Reversed):  000, 100, 010, 110, 001, 101, 011, 111 (0,4,2,6,1,5,3,7)
        let expected = vec![0, 4, 2, 6, 1, 5, 3, 7];
        let mut actual = Vec::new();

        while let Some(index) = order.next() {
            actual.push(index);
        }

        assert_eq!(actual, expected, "MSB order sequence is incorrect");
    }

    #[test]
    fn test_msb_completeness_and_uniqueness() {
        let num_vars = 5;
        let expected_count = 1 << num_vars;
        let order = SignificantBitOrder::new(num_vars);
        
        let results: Vec<usize> = order.collect();

        // 1. Check total count
        assert_eq!(results.len(), expected_count);

        // 2. Check uniqueness and coverage
        // Every value from 0 to 2^n - 1 must appear exactly once
        let mut sorted_results = results.clone();
        sorted_results.sort();
        for i in 0..expected_count {
            assert_eq!(sorted_results[i], i, "Value {} is missing or duplicated", i);
        }
    }

    #[test]
    fn test_msb_single_variable() {
        let mut order = SignificantBitOrder::new(1);
        assert_eq!(order.next(), Some(0));
        assert_eq!(order.next(), Some(1));
        assert_eq!(order.next(), None);
    }

    #[test]
    fn test_msb_zero_variables() {
        let mut order = SignificantBitOrder::new(0);
        // 2^0 = 1 element (the value 0)
        assert_eq!(order.next(), Some(0));
        assert_eq!(order.next(), None);
    }

    #[test]
    fn test_next_value_logic_directly() {
        // Test n=4 specifically for a few transitions
        // 1100 (12) -> next should flip MSB (bit 3) -> 0100 (4)
        // Then flip bit 2 -> 1100 is not it... let's trace:
        // 1100: i=3 -> result 0100. (0100 >> 3) == 1 is false.
        //       i=2 -> result 0000. (0000 >> 2) == 1 is false.
        //       i=1 -> result 0010. (0010 >> 1) == 1 is true. Break.
        // result = 2 (0010)
        assert_eq!(SignificantBitOrder::next_value_in_msb_order(12, 4), 2);
    }
}