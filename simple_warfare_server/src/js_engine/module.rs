use boa_engine::prelude::*;

#[derive(Debug)]
pub struct ModModule {
    pub module: Module,
    pub classes: Vec<String>,
}

impl ModModule {
    pub fn new(module: Module, classes: Vec<String>) -> Self {
        Self { module, classes }
    }
}
