use super::super::business::*;
use super::types::*;

impl Business for InnerState {
    fn business_example_query(&self) -> String {
        self.example_data.clone()
    }

    fn business_example_update(&mut self, test: String) {
        self.example_data = test
    }

    fn business_example_count_query(&self) -> u64 {
        self.example_count
    }

    fn business_example_count_update(&mut self, value: u64) {
        self.example_count = value
    }
}
