extern crate rand;

use rand::Rng;
use std::f64::consts::PI;

// classe point
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}
// classe panneau solaire
#[derive(Debug)]
struct SolarPanel {
    position: Point,
    temperature: f64,
    energy_level: f64,
    connectivity: i32,
}

impl SolarPanel {
    fn new(position: Point, temperature: f64, energy_level: f64, connectivity: i32) -> SolarPanel {
        SolarPanel {
            position,
            temperature,
            energy_level,
            connectivity,
        }
    }
}
// Fonction pour calculer l'énergie totale du système basée sur la distance entre les points.
fn calculate_energy(solar_panels: &[SolarPanel]) -> f64 {
    let mut energy = 0.0;
    for i in 0..solar_panels.len() {
        for j in i + 1..solar_panels.len() {
            energy += 1.0 / solar_panels[i].position.distance(&solar_panels[j].position);
        }
    }
    energy
}

// Fonction pour réarranger les points en utilisant l'algorithme de recuit simulé.
fn rearrange_panels(solar_panels: &mut Vec<SolarPanel>, temperature: f64) {
    let mut rng = rand::thread_rng();
    let current_energy = calculate_energy(solar_panels);

    for _ in 0..1000 {
        let i = rng.gen_range(0..solar_panels.len());
        let j = rng.gen_range(0..solar_panels.len());
        if i != j {


            let dx = rng.gen_range(-0.1..0.1);
            let dy = rng.gen_range(-0.1..0.1);
            let dz = rng.gen_range(-0.1..0.1);

            solar_panels[i].position.x += dx;
            solar_panels[i].position.y += dy;
            solar_panels[i].position.z += dz;

            let new_energy = calculate_energy(solar_panels);

            let delta_energy = new_energy - current_energy;

            if delta_energy > 0.0 && rng.gen_range(0.0..1.0) > (-delta_energy / temperature).exp() {
                solar_panels[i].position.x -= dx;
                solar_panels[i].position.y -= dy;
                solar_panels[i].position.z -= dz;
            }
        }
    }
}

fn main() {
    const NUM_PANELS: usize = 10;
    const INITIAL_TEMPERATURE: f64 = 100.0;
    const FINAL_TEMPERATURE: f64 = 0.01;

    let mut solar_panels = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_PANELS {
        let theta = rng.gen_range(0.0..2.0 * PI);
        let phi = rng.gen_range(0.0..PI);
        let radius = rng.gen_range(1.0..10.0);

        let x = radius * theta.sin() * phi.cos();
        let y = radius * theta.sin() * phi.sin();
        let z = radius * theta.cos();

        let position = Point::new(x, y, z);
        let temperature = rng.gen_range(20.0..30.0);
        let energy_level = rng.gen_range(70.0..100.0);
        let connectivity = rng.gen_range(80..100);

        let solar_panel = SolarPanel::new(position, temperature, energy_level, connectivity);

        solar_panels.push(solar_panel);
    }

    let mut temperature = INITIAL_TEMPERATURE;

    while temperature > FINAL_TEMPERATURE {
        rearrange_panels(&mut solar_panels, temperature);
        temperature *= 0.99;
    }

    println!("{:?}", solar_panels);
}
