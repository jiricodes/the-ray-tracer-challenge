pub mod prelude;

pub mod canvas;
pub mod color;
pub mod matrix;
pub mod vec4;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
