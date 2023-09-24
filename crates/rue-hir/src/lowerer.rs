use crate::error::Error;

pub struct Lowerer {
    pub(crate) errors: Vec<Error>,
}

impl Lowerer {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn errors(self) -> Vec<Error> {
        self.errors
    }
}
