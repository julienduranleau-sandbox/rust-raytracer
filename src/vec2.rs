pub mod vec2 {

    use std::ops;

    #[derive(Debug)]
    pub struct Vec2 {
        x: f64,
        y: f64,
    }

    impl Vec2 {
        fn new(x: f64, y: f64) -> Vec2 {
            Vec2 { x, y }
        }
    }
}
