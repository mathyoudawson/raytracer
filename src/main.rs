mod math;

fn main() {
    const WIDTH: u32 = 100;
    const HEIGHT: u32= 100;

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
                ],
                color: math::Rgb(0.3, 0.6, 0.2),
            }
        ]    
    };

    let mut frame_buffer = build_bitmap(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
          /**  let ray = math::Ray {
                origin: math::Vector {
                    x: x as f32,
                    y: y as f32,
                    z: 0.0,
                },
                direction: math::Vector {
                    x: x as f32,
                    y: y as f32,
                    z: 0.0,
                    },
                    }; **/

            let origin = math::Vector {
                x: 5.0,
                y: 5.0,
                z: 0.0,
            };

            let ray = math::Ray {
                origin: origin * 20.0,
                direction: - origin,
            };



            if let Some(tri) = world.get_intersecting_triangle(ray) {
                frame_buffer[(y * WIDTH + x) as usize] = tri.color;
            }
            else {
                frame_buffer[(y * WIDTH + x) as usize] = math::Rgb::WHITE;
            }
        }
    }

    image::save_buffer_with_format("frame-buffer.png",
                       &convert_bitmap_to_image(&frame_buffer[..])[..],
                       WIDTH,
                       HEIGHT,
                       image::ColorType::RGB(8),
                       image::ImageFormat::PNG).unwrap();
}

fn build_bitmap(width: u32, height: u32) -> Vec<math::Rgb> {
    vec![math::Rgb::WHITE; (width * height) as usize]
}

fn convert_bitmap_to_image(bitmap: &[math::Rgb]) -> Vec<u8> {
    bitmap.iter()
        .flat_map(|x| vec![x.0, x.1, x.2])
        .map(|x| (x * 255.0) as u8)
        .collect()
}

#[derive(Clone, PartialEq, Debug)]
pub struct World {
    triangles: Vec<math::Triangle>
}

pub struct Canvas {
    pub bitmap: Vec<math::Rgb>
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
                    ],
                    color: math::Rgb(0.3, 0.6, 0.2),
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
                    ],
                    color: math::Rgb(0.3, 0.6, 0.2),
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
