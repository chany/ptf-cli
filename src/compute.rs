use num::complex::{Complex64, ComplexFloat};

pub fn is_converge(mut z: Complex64, max_iter: usize, escape: f64) -> bool {
    let mut converge = true;
    let c_val = z;
    for _ in 0..max_iter {
        z = c_val.powc(z);
        if z.abs() > escape {
            converge = false;
            break;
        }
    }

    return converge;
}
