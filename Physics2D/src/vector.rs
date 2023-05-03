pub mod vector {
    use std::{ops, iter::Sum};

    use serde::{Serialize, Serializer, ser::SerializeTuple};

    #[derive(Copy, Clone)]
    pub struct Vec2{
        pub x: f64,
        pub y: f64,
    }

    impl Vec2 {
        pub fn new(x: f64, y: f64) -> Vec2{
            Vec2 { x: (x), y: (y)}
        }
        pub fn length(&self) -> f64 {
            return f64::sqrt(self.x*self.x + self.y*self.y);
        }
        pub fn squared_length(&self) -> f64 {
            return self.x*self.x + self.y*self.y;
        }
        pub fn dot(v1:Vec2,v2: Vec2) -> f64 {
            return v1.x*v2.x + v1.y*v2.y;
        }
        pub fn unit_vector(v1: Vec2) -> Vec2{
            return v1 / v1.length();
        }
        pub fn cross(v1: Vec2, v2: Vec2) -> f64 {
            return v1.x*v2.y - v1.y*v2.x;
        }


    }
    impl Serialize for Vec2 {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut tuple = serializer.serialize_tuple(2)?;
            tuple.serialize_element(&self.x)?;
            tuple.serialize_element(&self.y)?;
            tuple.end()
        }
    }
    impl ops::Add<Vec2> for Vec2 {
        type Output = Vec2;

        fn add(self, _rhs: Vec2) -> Self::Output {
            Vec2::new(self.x+_rhs.x, self.y+_rhs.y)
        }
    }
    impl ops::Neg<> for Vec2 {
        type Output = Vec2;

        fn neg(self) -> Self::Output {
            Vec2::new(-self.x, -self.y)
        }
    }
    impl ops::Sub<Vec2> for Vec2 {
        type Output = Vec2;

        fn sub(self, _rhs: Vec2) -> Self::Output {
            Vec2::new(self.x-_rhs.x, self.y-_rhs.y)
        }
    }
    impl ops::Mul<Vec2> for Vec2 {
        type Output = Vec2;

        fn mul(self, _rhs: Vec2) -> Self::Output {
            Vec2::new(self.x*_rhs.x, self.y*_rhs.y)
        }
    }
    impl ops::Mul<Vec2> for f64 {
        type Output = Vec2;
        fn mul(self, rhs: Vec2) -> Self::Output {
            Vec2::new(rhs.x*self, rhs.y*self)
        }
    }
    impl ops::Div<Vec2> for Vec2 {
        type Output = Vec2;
        fn div(self, _rhs: Vec2) -> Self::Output {
            Vec2::new(self.x / _rhs.x, self.y / _rhs.y)
        }
    }
    impl ops::Mul<f64> for Vec2 {
        type Output = Vec2;
        fn mul(self, rhs: f64) -> Self::Output {
            Vec2::new(self.x*rhs, self.y*rhs)
        }
    }
    impl ops::Div<f64> for Vec2 {
        type Output = Vec2;
        fn div(self, rhs: f64) -> Self::Output {
            Vec2::new(self.x/rhs, self.y/rhs)
        }
    }
    impl ops::AddAssign<Vec2> for Vec2 {
        fn add_assign(&mut self, rhs: Vec2) {
            self.x+=rhs.x;
            self.y+=rhs.y;
        }
    }
    impl ops::MulAssign<Vec2> for Vec2 {
        fn mul_assign(&mut self, rhs: Vec2) {
            self.x *= rhs.x;
            self.y *= rhs.y;
        }
    }
    impl ops::DivAssign<Vec2> for Vec2 {
        fn div_assign(&mut self, rhs: Vec2) {
            self.x /= rhs.x;
            self.y /= rhs.y;
        }
    }
    impl ops::SubAssign<Vec2> for Vec2 {
        fn sub_assign(&mut self, rhs: Vec2) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }
    impl ops::MulAssign<f64> for Vec2 {
        fn mul_assign(&mut self, rhs: f64) {
            self.x *= rhs;
            self.y *= rhs;
        }
    }
    impl ops::DivAssign<f64> for Vec2 {
        fn div_assign(&mut self, rhs: f64) {
            self.x /= rhs;
            self.y /= rhs;
        }
    }
    impl Sum<Self> for Vec2 {
        fn sum<I>(iter: I) -> Self
        where
            I: Iterator<Item = Self>,
            {
                iter.fold(Self {x: 0.0, y: 0.0}, |a, b| Self {
                    x: a.x + b.x,
                    y: a.y + b.y,
                })
            }
    }
}