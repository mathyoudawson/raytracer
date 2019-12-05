use std::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Triangle {
    pub points: [Vector; 3]
}

pub type Scalar = f32;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rgb (pub f32, pub f32, pub f32);

impl Rgb {
    pub const WHITE: Self = Rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Rgb(0.0, 0.0, 0.0);
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x, 
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Mul<Scalar> for Vector {
    type Output = Vector;

    fn mul(self, other: Scalar) -> Vector {
        Vector {
            x: self.x * other, 
            y: self.y * other,
            z: self.z * other,
        }
    }
}


impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: - self.x, 
            y: - self.y,
            z: - self.z,
        }
    }
}

impl Vector {
    pub fn cross_product(self, other: Vector) -> Vector {
        let cx = self.y * other.z - self.z * other.y;
        let cy = self.z * other.x - self.x * other.z;
        let cz = self.x * other.y - self.y * other.x;

        Vector {
            x: cx,
            y: cy,
            z: cz,
        }
    }

    pub fn dot_product(self, other: Vector) -> Scalar {
        self.x * other.x + self.y * other.y
    }
}

impl Ray {
    pub fn intersects_triangle(&self, in_triangle: &Triangle) -> Option<Vector>
    {
        let vertex0: Vector = in_triangle.points[0];
        let vertex1: Vector = in_triangle.points[1];  
        let vertex2: Vector = in_triangle.points[2];

        let edge1 = vertex1 - vertex0;
        let edge2 = vertex2 - vertex0;
        let h = self.direction.cross_product(edge2);
        let a = edge1.dot_product(h);

        if a > -std::f32::EPSILON && a < std::f32::EPSILON { 
            return None;
        }

        let f = 1.0/a;
        let s = self.origin - vertex0;
        let u = f * s.dot_product(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross_product(edge1);
        let v = f * self.direction.dot_product(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t: f32 = f * edge2.dot_product(q);

        if t > std::f32::EPSILON && t < 1.0 / std::f32::EPSILON // ray intersection
        {
            Some(self.origin + self.direction * t)
        }
        else // This means that there is a line intersection but not a ray intersection. 
        {
            None
        }
    }
}
