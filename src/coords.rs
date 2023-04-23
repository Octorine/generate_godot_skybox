// Spherical Coordinates in this program are defined as:
//  theta - vertical angle from the z axis ccw
//  phi - horizontal angle from the x axis  ccw
//  rho - distance from the origin
//
// Cartesian Coordinates are as follows:
//  x - horizontal with positive to the right
//  y - vertical with positive up
//  z - horizontal with positive forwards

use std::f64::consts::PI;

pub fn sphere_to_cart(theta: f64, phi: f64, rho: f64) -> [f64; 3] {
    let phi = PI * 0.5 - phi;
    let x = rho * theta.cos() * phi.sin();
    let y = rho * phi.cos();
    let z = rho * theta.sin() * phi.sin();
    [x, y, z]
}

pub fn cart_to_sphere(x: f64, y: f64, z: f64) -> [f64; 3] {
    let rho = (x * x + y * y + z * z).sqrt();
    let theta = if x == 0.0 { 0.0 } else { (z / x).atan() };
    let phi = if rho == 0.0 {
        0.0
    } else {
        PI * 0.5 - (y / rho).acos()
    };
    [theta, phi, rho]
}

#[cfg(test)]

mod tests {
    use crate::coords::{cart_to_sphere, sphere_to_cart};
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
            is_close(sphere_to_cart(0.0, 0.0, 1.0), [1.0, 0.0, 0.0]),
            "{:?} != {:?}",
            sphere_to_cart(0.0, 0.0, 1.0),
            [1.0, 0.0, 0.0]
        );
        assert!(is_close(sphere_to_cart(PI, 0.0, 1.0), [-1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_c_to_s() {
        assert!(
            is_close([0.0, 0.0, 1.0], cart_to_sphere(1.0, 0.0, 0.0)),
            "{:?} != {:?}",
            [0.0, 0.0, 1.0],
            cart_to_sphere(1.0, 0.0, 0.0)
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

    fn test_roundtrip_s_c_s(theta: f64, phi: f64, rho: f64) {
        let [x, y, z]  = sphere_to_cart(theta, phi, rho);
        assert!(
            is_close([theta, phi, rho], cart_to_sphere(x, y, z)),
            "{:?} != {:?}",
            (theta, phi, rho),
            cart_to_sphere(x, y, z)
        );
    }
    fn test_roundtrip_c_s_c(x: f64, y: f64, z: f64) {
        let [theta, phi, rho] = cart_to_sphere(x, y, z);
        assert!(
            is_close([x, y, z], sphere_to_cart(theta, phi, rho)),
            "{:?} != {:?}",
            (x, y, z),
            sphere_to_cart(rho, theta, phi)
        );
    }
}
