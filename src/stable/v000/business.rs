use super::super::business::*;
use super::types::*;

impl Business for InnerState {}

#[allow(clippy::panic)] // ? 允许回滚
#[allow(clippy::unwrap_used)] // ? 允许回滚
#[allow(clippy::expect_used)] // ? 允许回滚
impl MutableBusiness for InnerState {}
