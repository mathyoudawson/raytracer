mod math;

fn main() {
    let world = World {
        triangles: vec![
            math::Triangle {
                points: [
                    math::Vector  {
                        x: 3.2,
                        y: 4.9,
                        z: 23.00001,
                    },
                    math::Vector  {
                        x: 3.2,
                        y: 4.9,
                        z: 3.00001,
                    },
                    math::Vector  {
                        x: 3.2,
                        y: 4.9,
                        z: 2.00001,
                    },
                ]
            }
        ]
    };

    let ray = math::Ray {
        origin: math::Vector {
            x: 3.2,
            y: 4.9,
            z: 2.00001,
        },
        direction: math::Vector {
            x: 3.2,
            y: 4.9,
            z: 2.00001,
        },
    };

   let tri = world.get_intersecting_triangle(ray);
   println!("Triangle: {:#?}", tri);
}


#[derive(Clone, PartialEq, Debug)]
pub struct World {
    triangles: Vec<math::Triangle>
}

impl World {
    pub fn get_intersecting_triangle(&self, ray: math::Ray) -> Option<math::Triangle>
    {
    for triangle in self.triangles.iter() {
            match ray.intersects_triangle(triangle) {
                Some(_) => {
                    return Some(*triangle); 
                },
                None => {},
            }
    }
      None 
    }
}

#[cfg(test)]
mod test {
    use crate::math;
    use super::*;

    #[test]
    fn basic_intersection(){
        let world = World {
            triangles: vec![
                math::Triangle {
                    points: [
                        math::Vector  {
                            x: 0.0,
                            y: -1.00,
                            z: 0.0,
                        },
                        math::Vector  {
                            x: 0.0,
                            y: 1.00,
                            z: -1.00,
                        },
                        math::Vector  {
                            x: 0.0,
                            y: 1.00,
                            z: 1.00,
                        },
                    ]
                }
            ]
        };

        let origin = math::Vector {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        };

        let ray = math::Ray {
            origin,
            direction: - origin,
        };

        let tri = world.get_intersecting_triangle(ray);
        assert_eq!(tri, Some(world.triangles[0]));
    }

    #[test]
    fn negative_intersection(){
        let world = World {
            triangles: vec![
                math::Triangle {
                    points: [
                        math::Vector  {
                            x: 1.0,
                            y: 1.00,
                            z: 1.0,
                        },
                        math::Vector  {
                            x: 0.0,
                            y: 1.00,
                            z: -1.00,
                        },
                        math::Vector  {
                            x: 0.0,
                            y: 0.00,
                            z: 1.00,
                        },
                    ]
                }
            ]
        };

        let origin = math::Vector {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        };

        let ray = math::Ray {
            origin,
            direction: - origin,
        };

        let tri = world.get_intersecting_triangle(ray);
        assert_eq!(tri, None);

    }
}
