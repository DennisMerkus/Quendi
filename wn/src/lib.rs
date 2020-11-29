#[macro_use] extern crate lazy_static;

pub mod multi;
pub mod pos;
pub mod princeton;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
