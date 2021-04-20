//! A simple example of interpolation using basis functions.

use nalgebra::DVector;
use std::time::Instant;
use rbf_interp::{Basis, Scatter};

fn main() {
    let n_sample = 2000;
    let mut xs = Vec::with_capacity(n_sample);
    let mut ys = Vec::with_capacity(n_sample);
    for i in 0..n_sample {
        let x = 0.2 * (i as f64);
        let y = x.sin();
        xs.push(DVector::from_vec(vec![x]));
        ys.push(DVector::from_vec(vec![y]));
    }
    println!("RBF creation ...");
    let now = Instant::now();
    let scatter = Scatter::create(xs, ys, Basis::PolyHarmonic(2), 2);
    println!(" ... in {}ms", now.elapsed().as_millis());
    for i in 0..500 {
        let x = 0.004 * (i as f64);
        let y = scatter.eval(DVector::from_vec(vec![x]));
        //println!("{:5.3} {:5.3} {:5.3}", x, y[0], x.sin());
    }
}
