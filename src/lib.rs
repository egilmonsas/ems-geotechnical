#![allow(dead_code)]
#![warn(clippy::pedantic)]

pub mod hydro;
pub mod profile;
pub mod soil;

#[must_use]
pub fn linspace(min: f64, max: f64, n: usize) -> Vec<f64> {
    let delta = delta(min, max, n);

    (0..n).map(|i| min + delta * i as f64).collect()
}
#[must_use]
pub fn delta(min: f64, max: f64, n: usize) -> f64 {
    (max - min) / (n - 1) as f64
}
