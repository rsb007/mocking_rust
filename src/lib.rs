#![feature(proc_macro_hygiene)]
extern crate mocktopus;
use mocktopus::macros::*;

#[mockable]
mod sample {

    pub fn world() -> &'static str {
        "world"
    }

    pub fn hello_world() -> String {
        format!("Hello {}!", world())
    }
}

#[cfg(test)]

mod test {
    use super::*;
    use mocktopus::mocking::*;

    #[test]
    fn test_1 (){

        sample::world.mock_safe(|| MockResult::Return("asd"));
        assert_eq!("asd",sample::world());
    }
}
/*
#[mockable]
mod hello_world {
    pub fn world() -> &'static str {
        "world"
    }

    pub fn hello_world() -> String {
        format!("Hello {}!", world())
    }
}

#[test]
fn mock_test() {
    use mocktopus::mocking::*;
    hello_world::world.mock_safe(|| MockResult::Return("mocking"));

    assert_eq!("Hello mocking!", hello_world::hello_world());
}*/
