use wasm_bindgen::prelude::*;

const G: f64 = 9.8;

#[wasm_bindgen]
pub fn potential_energy(m: f64, h: f64) -> f64 {
    m * G * h
}

#[wasm_bindgen]
pub fn kinetic_energy(m: f64, v: f64) -> f64 {
    (m * v.powi(2)) / 2.0
}

#[test]
fn we_get_the_potential_energy() {
    assert_eq!(potential_energy(10.0, 2.0), 196.0);
}

#[test]
fn we_get_the_kinetic_energy() {
    assert_eq!(kinetic_energy(10.0, 2.0), 20.0);
}
