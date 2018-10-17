pub mod vec3 {
    use std::ops;

    #[derive(Debug)]
    pub struct Vec3 {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Vec3 {
        fn new(x: f64, y: f64, z: f64) -> Vec3 {
            Vec3 { x, y, z }
        }
        
        fn mag(&self) -> f64 {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }
    /*
        fn norm(&self) -> Vec3 {
            let mag = self.mag();
            if mag > 0 {

            }
        }
        */
    }

    impl ops::Add for Vec3 {
        type Output = Vec3;

        fn add(self, other: Vec3) -> Vec3 {
            Vec3::new(
                self.x + other.x,
                self.y + other.y,
                self.z + other.z,
            )
        }
    }

    impl<'a > ops::Add<&'a Vec3> for Vec3 {
        type Output = Vec3;

        fn add(self, other: &Vec3) -> Vec3 {
            Vec2::new(
                self.x + other.x,
                self.y + other.y,
                self.z + other.z,
            )
        }
    }

    impl<'a, 'b> ops::Add<&'b Vec3> for &'a Vec3 {
        type Output = Vec3;

        fn add(self, other: &'b Vec3) -> Vec3 {
            Vec2::new(
                self.x + other.x,
                self.y + other.y,
                self.z + other.z,
            )
        }
    }
}
