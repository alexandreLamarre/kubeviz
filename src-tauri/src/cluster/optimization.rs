use core::ops::Fn;
use rayon::prelude::*;
use std::error::Error;
use std::ops::{Add, Div, Mul, Sub};

//FIXME: this is a hack to get the somethig working
type Force = Vec<f32>;
type Vertices = Vec<Vec<f32>>;

/// Translates an Object into a vector of weights
pub trait IntoForce {
    fn into_force(&self) -> Vec<f32>;
}
pub trait IntoPair {
    fn link(&self) -> (u32, u32);
}

#[derive(Default)]
struct Cluster {
    vertices: Vec<Vec<f32>>,
    edges: Vec<Vec<u32>>,
}

impl Cluster {
    fn from_iter<T>(iter: Box<dyn Iterator<Item = T>>)
    where
        T: IntoPair + IntoForce,
    {
        let mut vertices = Vec::new();
        let mut edges = Vec::new();
        for item in iter {
            let (e1, e2) = item.link();
            vertices.push(item.into_force());
            edges.push(vec![e1, e2]);
        }
    }
}

struct ForceAtlas2 {
    pub k: f32,
    pub c: f32,
    pub p: f32,
    /// Convergence bound for the total energy of the layout
    pub epislon: f32,
    pub max_iterations: usize,
    /// In milliseconds
    pub max_time: f32,
    pub max_force: f32,
    pub theta: f32,
    pub iterations: usize,
    pub dissuadeHub: bool,
    pub lin_log: bool,
    pub f_attract: Box<dyn Fn(Force, Force) -> Force>,
    pub f_repel: Box<dyn Fn(Force, Force) -> Force>,
}

impl ForceAtlas2 {
    /// Optimize cluster layout by using Force Atlas 2 algorithm.
    /// Returns a new cluster layout
    pub fn run(self, clus: Cluster) -> Result<Cluster, Box<dyn Error>> {
        // let len = clus.vertices.count();

        // Calculate attractive forces

        // Calculate repulsive forces

        // Calculate gravity forces

        // Update global speeds

        // Update trajectory swing

        Ok(clus)
    }
}

impl Default for ForceAtlas2 {
    fn default() -> Self {
        ForceAtlas2 {
            k: 1.0,
            c: 1.0,
            p: 1.0,
            epislon: 0.0001,
            max_iterations: 1000,
            max_time: 1000.0,
            max_force: 0.1,
            theta: 0.1,
            iterations: 0,
            dissuadeHub: false,
            lin_log: false,
            f_attract: Box::new(|a, b| a),
            f_repel: Box::new(|a, b| a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs() {
        let _ = ForceAtlas2::default();
    }

    #[test]
    fn it_runs() {
        let _ = ForceAtlas2::default().run(Cluster::default());
    }
}
