// Spherical Coordinates in this program are defined as:
//  azimuthal - vertical angle from the z axis ccw
//  polar - horizontal angle from the x axis  ccw
//  radius - distance from the origin
//
// Cartesian Coordinates are as follows:
//  x - horizontal with positive to the right
//  y - vertical with positive up
//  z - horizontal with positive forwards

use std::f64::consts::PI;

#[derive(Debug)]
pub struct Spherical {
    pub azimuthal: f64,
    pub polar: f64,
    pub radius: f64,
}

impl Spherical {
    pub fn to_rectangular(&self) -> Rectangular {
        let phi = PI * 0.5 - self.azimuthal;
        let x = self.radius * self.polar.cos() * phi.sin();
        let y = self.radius * phi.cos();
        let z = self.radius * self.polar.sin() * phi.sin();
        Rectangular { x, y, z }
    }
    pub fn arr(&self) -> [f64; 3] {
        [self.azimuthal, self.polar, self.radius]
    }
}
#[derive(Debug)]
pub struct Rectangular {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Rectangular {
    pub fn to_spherical(&self) -> Spherical {
        let radius = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        let polar = if self.x == 0.0 {
            0.0
        } else {
            (self.z / self.x).atan()
        };
        let azimuthal = if radius == 0.0 {
            0.0
        } else {
            PI * 0.5 - (self.y / radius).acos()
        };
        Spherical {
            azimuthal,
            polar,
            radius,
        }
    }
    pub fn arr(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}
#[cfg(test)]

mod tests {
    use crate::coords::{Rectangular, Spherical};
    use std::f64::consts::PI;

    fn is_close(a: [f64; 3], b: [f64; 3]) -> bool {
        let epsilon = 0.0001;
        (a[0] - b[0]).abs() < epsilon
            && (a[1] - b[1]).abs() < epsilon
            && (a[2] - b[2]).abs() < epsilon
    }
    #[test]
    fn test_s_to_c() {
        assert!(
            is_close(
                Spherical {
                    azimuthal: 0.0,
                    polar: 0.0,
                    radius: 1.0
                }
                .to_rectangular()
                .arr(),
                [1.0, 0.0, 0.0]
            ),
            "{:?} != {:?}",
            Spherical {
                azimuthal: 0.0,
                polar: 0.0,
                radius: 1.0
            }
            .to_rectangular()
            .arr(),
            [1.0, 0.0, 0.0]
        );
        assert!(is_close(
            Spherical {
                azimuthal: PI,
                polar: 0.0,
                radius: 1.0
            }
            .to_rectangular()
            .arr(),
            [-1.0, 0.0, 0.0]
        ));
    }

    #[test]
    fn test_c_to_s() {
        assert!(
            is_close(
                [0.0, 0.0, 1.0],
                Rectangular {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0
                }
                .to_spherical()
                .arr()
            ),
            "{:?} != {:?}",
            [0.0, 0.0, 1.0],
            Rectangular {
                x: 1.0,
                y: 0.0,
                z: 0.0
            }
            .to_spherical()
            .arr()
        );
    }

    #[test]
    fn test_coords() {
        test_roundtrip_c_s_c(1.0, 1.0, 1.0);
        test_roundtrip_c_s_c(0.1, 1.0, 1.0);
        test_roundtrip_c_s_c(0.3, 0.2, 1.5);
        test_roundtrip_s_c_s(1.0, 1.0, 1.0);
        test_roundtrip_s_c_s(0.1, 1.0, 1.0);
        test_roundtrip_s_c_s(0.3, 0.2, 1.5);
    }

    fn test_roundtrip_s_c_s(azimuthal: f64, polar: f64, radius: f64) {
        let sphere = Spherical {
            azimuthal,
            polar,
            radius,
        };
        assert!(
            is_close(sphere.arr(), sphere.to_rectangular().to_spherical().arr()),
            "{:?} != {:?}",
            sphere,
            sphere.to_rectangular().to_spherical()
        );
    }
    fn test_roundtrip_c_s_c(x: f64, y: f64, z: f64) {
        let rect = Rectangular { x, y, z };
        assert!(
            is_close(rect.arr(), rect.to_spherical().to_rectangular().arr()),
            "{:?} != {:?}",
            rect.arr(),
            rect.to_spherical().to_rectangular().arr()
        );
    }
}
