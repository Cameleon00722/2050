extern crate rand;

use rand::Rng;
use std::f64::consts::PI;

// Définir une structure Point pour représenter les coordonnées d'un point.
#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}
// Implémenter des méthodes pour la structure Point.
impl Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}

// Fonction pour calculer l'énergie totale du système basée sur la distance entre les points.
fn calculate_energy(points: &[Point]) -> f64 {
    let mut energy = 0.0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            energy += 1.0 / points[i].distance(&points[j]);
        }
    }
    energy
}

// Fonction pour réarranger les points en utilisant l'algorithme de recuit simulé.
fn rearrange_points(points: &mut Vec<Point>, temperature: f64) {
    let mut rng = rand::thread_rng();
    let current_energy = calculate_energy(points);

    for _ in 0..1000 {
        let i = rng.gen_range(0..points.len());
        let j = rng.gen_range(0..points.len());
        if i != j {

            let dx = rng.gen_range(-0.1..0.1);
            let dy = rng.gen_range(-0.1..0.1);
            let dz = rng.gen_range(-0.1..0.1);

            points[i].x += dx;
            points[i].y += dy;
            points[i].z += dz;

            let new_energy = calculate_energy(points);

            let delta_energy = new_energy - current_energy;

            if delta_energy > 0.0 && rng.gen_range(0.0..1.0) > (-delta_energy / temperature).exp() {
                points[i].x -= dx;
                points[i].y -= dy;
                points[i].z -= dz;
            }
        }
    }
}

fn main() {
    const NUM_POINTS: usize = 10;
    const INITIAL_TEMPERATURE: f64 = 100.0;
    const FINAL_TEMPERATURE: f64 = 0.01;

    let mut points = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_POINTS {
        let theta = rng.gen_range(0.0..2.0 * PI);
        let phi = rng.gen_range(0.0..PI);
        let radius = rng.gen_range(1.0..10.0);

        let x = radius * theta.cos() * phi.sin();
        let y = radius * theta.sin() * phi.sin();
        let z = radius * phi.cos();

        points.push(Point::new(x, y, z));
    }

    let mut temperature = INITIAL_TEMPERATURE;

    while temperature > FINAL_TEMPERATURE {
        rearrange_points(&mut points, temperature);
        temperature *= 0.99;
    }

    println!("{:?}", points);
}