extern crate rand;

use rand::Rng;
use std::f64::consts::PI;

// classe point
#[derive(Debug)]
#[derive(Clone)]
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
#[derive(Clone)]
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
fn rearrange_panels_hyperion(solar_panels: &mut Vec<SolarPanel>){

    let mut rng = rand::thread_rng();
    let cloned_solar = solar_panels.clone();

    for panel in solar_panels{

        let mut current_energy = 0.0;
        for i in 0..cloned_solar.clone().len() {
            for j in i + 1..cloned_solar.len() {
                current_energy += 1.0 / cloned_solar[i].position.distance(&cloned_solar[j].position);
            }
        }


        if panel.energy_level < 5.{
            eprintln!("energy level too low")
        }

        for _ in 0..100 {
            let dx = rng.gen_range(-0.1..0.1);
            let dy = rng.gen_range(-0.1..0.1);
            let dz = rng.gen_range(-0.1..0.1);


            panel.position.x += dx;
            panel.position.y += dy;
            panel.position.z += dz;

            let mut new_energy = 0.0;
            for i in 0..cloned_solar.clone().len() {
                for j in i + 1..cloned_solar.len() {
                    new_energy += 1.0 / cloned_solar[i].position.distance(&cloned_solar[j].position);
                }
            }

            let delta_energy = new_energy - current_energy;

            if delta_energy > 0.0 && rng.gen_range(0.0..1.0) > (-delta_energy / panel.temperature).exp() {
                panel.position.x -= dx;
                panel.position.y -= dy;
                panel.position.z -= dz;
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
        rearrange_panels_hyperion(&mut solar_panels);
        temperature *= 0.99;
    }

    println!("{:?}", solar_panels);
}
