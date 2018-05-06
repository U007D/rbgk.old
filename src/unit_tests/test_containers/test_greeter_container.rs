use di::Container;
use self::mocks::MockGreeter;
use super::*;

pub struct TestGreeterContainer {}

impl Container for TestGreeterContainer {
    fn build() -> Self {
        Self {}
    }
}

impl TestGreeterContainer {
    pub fn resolve_greeter(&self) -> MockGreeter {
        MockGreeter::new()
    }
}
