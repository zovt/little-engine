#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate cgmath;
extern crate glutin;

pub mod objects;
pub mod transform;
pub mod controller;