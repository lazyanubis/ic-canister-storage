use super::super::business::*;
use super::types::*;

impl Business for InnerState {
    fn business_example_query(&self) -> String {
        self.example_data.clone()
    }

    fn business_example_update(&mut self, test: String) {
        self.example_data = test
    }
}
