use std::f64::consts::PI;
use rand::Rng;
use crate::{Point, SolarPanel};

pub fn create_hexagonal_pattern(num_panels: usize) -> Vec<SolarPanel> {
    let mut solar_panels = Vec::new();
    let mut rng = rand::thread_rng();

    // Paramètres pour la disposition hexagonale
    let hexagon_radius = 1.0;  // Rayon de l'alvéole d'abeille
    let angle_offset = PI / 6.0;  // Décalage d'angle pour chaque panneau

    for i in 0..num_panels {
        let theta = 2.0 * PI * (i as f64) / (num_panels as f64);
        let rotated_theta = theta + angle_offset;

        // Appliquer le décalage d'angle pour créer la structure hexagonale
        let x = hexagon_radius * rotated_theta.cos();
        let y = hexagon_radius * rotated_theta.sin();

        let z = rng.gen_range(1.0..10.0);  // Réglez la coordonnée z en fonction de vos besoins

        let position = Point::new(x, y, z);
        let temperature = rng.gen_range(20.0..30.0);
        let energy_level = rng.gen_range(70.0..100.0);
        let connectivity = rng.gen_range(80..100);

        let solar_panel = SolarPanel::new(position, temperature, energy_level, connectivity, 1.);
        solar_panels.push(solar_panel);
    }

    solar_panels
}