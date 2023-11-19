extern crate rand;

use rand::Rng;
use std::f64::consts::PI;


// classe point
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
struct SolarPanel {
    position: Point,
    temperature: f64,
    energy_level: f64,
    connectivity: i32,
    thruster: f64, // puissance de déplacement
}

impl SolarPanel {
    fn new(position: Point, temperature: f64, energy_level: f64, connectivity: i32, thruster: f64) -> SolarPanel {
        SolarPanel {
            position,
            temperature,
            energy_level,
            connectivity,
            thruster,

        }
    }
}

#[derive(Debug, Clone)]
struct SolarSwarm {
    name: String,
    solar_panels: Vec<SolarPanel>,
}

impl SolarSwarm {
    fn new(name: &str, solar_panels: Vec<SolarPanel>) -> SolarSwarm {
        SolarSwarm {
            name: name.to_string(),
            solar_panels,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Planete {
    position: Point,
    G_force: f64,
    Mass: f64,
    Orbital_period: f64,
}

impl Planete {
    fn new(position: Point, G_force: f64, Mass: f64, Orbital_period: f64) -> Planete {
        Planete {
            position,
            G_force,
            Mass,
            Orbital_period,

        }
    }

    fn calculate_geostationary_orbit_parameters(orbital_period: f64, G: f64, MERCURY_MASS: f64) -> (f64, f64) {
        let radius = ((G * MERCURY_MASS * orbital_period.powi(2)) / (4.0 * PI.powi(2))).powf(1.0 / 3.0);
        let inclination = 0.0;  // Inclinaison proche de zéro pour une orbite géostationnaire

        (radius, inclination)
    }

}


// Fonction pour réarranger les points en utilisant l'algorithme de recuit simulé.
fn rearrange_panels_hyperion(solar_swarm: &mut SolarSwarm){

    let mut rng = rand::thread_rng();
    let cloned_solar = solar_swarm.solar_panels.clone();


    for panel in solar_swarm.solar_panels.iter_mut(){

        let close_neighbors = cloned_solar.clone()
            .iter()
            .filter(|other_panel| calculate_distance(&panel.position, &other_panel.position) < 20.)
            .count() as i32;

        panel.connectivity = close_neighbors;

        let mut current_energy = 0.0;
        for i in 0..cloned_solar.clone().len() {
            for j in i + 1..cloned_solar.len() {
                current_energy += 1.0 / cloned_solar[i].position.distance(&cloned_solar[j].position);
            }
        }

        //impossible à trigger on a energie infinie avec l'étoile
        if panel.energy_level < 5.{
            println!("energy level too low")
        }

        for _ in 0..100 {
            let dx = rng.gen_range(-panel.thruster..panel.thruster);
            let dy = rng.gen_range(-panel.thruster..panel.thruster);
            let dz = rng.gen_range(-panel.thruster..panel.thruster);



            panel.position.x += dx;
            panel.position.y += dy;
            panel.position.z += dz;

            let new_distance_to_star = calculate_distance(&Point::new(0.0, 0.0, 0.0), &panel.position);

            // Vérification de la distance de sécurité
            if new_distance_to_star < 6. {
                // Reculer le panneau solaire (ajuster selon votre logique)
                panel.position.x += dx;
                panel.position.y += dy;
                panel.position.z += dz;

                println!("collision star warning")
            }


            let mut new_energy = 0.0;
            for i in 0..cloned_solar.clone().len() {
                for j in i + 1..cloned_solar.len() {
                    new_energy += 1.0 / cloned_solar[i].position.distance(&cloned_solar[j].position);
                }
            }

            // Vérification de la distance entre les panneaux ( à modifier pas optimal )
            let min_distance = 2.0;
            for other_panel in &cloned_solar{
                while other_panel.position.distance(&panel.position) < min_distance {
                    println!("collision risk DETECTED");
                    // Éloigner les panneaux si la distance est inférieure à 2
                    panel.position.x += dx * 2.0;
                    panel.position.y += dy * 2.0;
                    panel.position.z += dz * 2.0;
                }
            }


            let delta_energy = new_energy - current_energy;

            if delta_energy > 0.0 && rng.gen_range(0.0..1.0) > (-delta_energy / panel.temperature).exp() {
                panel.position.x -= dx;
                panel.position.y -= dy;
                panel.position.z -= dz;
            }

            if panel.temperature > 1500. {
                println!("panel temperature too hight");
                println!("begin automatique correction");

                let ancient_coordinate = panel.clone();

                while panel.temperature > 1000.{  // modifier pour prendre en compte la force des thrusters
                    panel.position.x += 10.;
                    panel.position.y += 10.;
                    panel.position.z += 10.;

                    //ajouter un timer ici
                    //plus écoute de la sonde thermique
                    panel.temperature -= 15.;

                }

                // rester plus loins que la position d'origine pour éviter une autre surchauffe
                panel.position.x = ancient_coordinate.position.x - 5.;
                panel.position.y = ancient_coordinate.position.y - 5.;
                panel.position.z = ancient_coordinate.position.z - 5.;

                println!("alerte end");

            }

        }
    }


}

fn calculate_distance(star_position: &Point, panel_position: &Point) -> f64 {
    ((star_position.x - panel_position.x).powi(2) + (star_position.y - panel_position.y).powi(2) + (star_position.z - panel_position.z).powi(2)).sqrt()
}

fn calculate_solar_energy(reference_intensity: f64, reference_distance: f64, target_distance: f64, surface_area: f64) -> f64 {
    // Calculer l'intensité à la nouvelle distance
    let target_intensity = reference_intensity / (target_distance.powf(2.0) / reference_distance.powf(2.0));

    // Calculer l'énergie solaire reçue à la nouvelle distance
    let solar_energy = target_intensity * surface_area;

    solar_energy
}




fn main() {
    const NUM_PANELS: usize = 10; // nombre de panneau souhaité
    const INITIAL_TEMPERATURE: f64 = -270.424; // température du vide spatial
    const DANGER_TEMPERATURE: f64 = 1668.; // température de fusion du titane
    const STAR_DIAMETER: f64 = 4.; // taille de l'étoile autours duquel on gravite

    let mut solar_panels = Vec::new();
    let mut rng = rand::thread_rng();
    let orbit_distance = 2.;

    for _ in 0..NUM_PANELS {
        let theta = rng.gen_range(0.0..2.0 * PI); //génère un nombre aléatoire dans la plage spécifiée, ici de 0 à 2π dans le plan xy
        let phi = rng.gen_range(0.0..PI); //génère un nombre aléatoire dans la plage spécifiée, ici de 0 à π dans le plan ZX
        let radius = orbit_distance + STAR_DIAMETER;  // Distance de d'implantation des satellites

        let x = radius * theta.sin() * phi.cos();
        let y = radius * theta.sin() * phi.sin();
        let z = radius * theta.cos();

        let position = Point::new(x, y, z);
        let temperature = rng.gen_range(1300.0..1700.0);
        let energy_level = rng.gen_range(70.0..100.0);
        let connectivity = rng.gen_range(80..100);

        let solar_panel = SolarPanel::new(position, temperature, energy_level, connectivity, 1.);

        solar_panels.push(solar_panel);
    }

    let mut solar_swarm = SolarSwarm::new("Hyperion", solar_panels);

    // simulateur de déplacement aléatoire
    for _ in 0..10 {
        rearrange_panels_hyperion(&mut solar_swarm);
    }

    for panel in solar_swarm.solar_panels{
        println!("{:?}", panel)
    }


    /////////calcul tier pour un positionnement supposé en orbite de mercure

    const REFERENCE_INTENSITY: f64 = 1361.0;  // Intensité solaire moyenne à la distance de référence en W/m^2 (exemple)
    const REFERENCE_DISTANCE: f64 = 5.0e10;   // Distance de référence en mètres (exemple)
    const TARGET_DISTANCE: f64 = 5.0e10;      // Nouvelle distance (distance orbitale du satellite géostationnaire) en mètres (exemple)
    const SURFACE_AREA: f64 = 10.0;           // Surface du panneau solaire en m^2 (exemple)

    let energy_received = calculate_solar_energy(REFERENCE_INTENSITY, REFERENCE_DISTANCE, TARGET_DISTANCE, SURFACE_AREA);

    println!("Énergie solaire reçue à {} unités de distance : {}", TARGET_DISTANCE, energy_received);


    let mercury = Planete::new(Point::new(10., 10., 10.), 6.674e-11, 3011e23, 58.6 * 24.0 * 3600.0);

    //  ORBITAL_PERIOD: f64 = 58.6 * 24.0 * 3600.0;
    // Période orbitale en secondes (58.6 jours terrestres convertis)

    let (radius, inclination) = Planete::calculate_geostationary_orbit_parameters(mercury.Orbital_period, mercury.G_force, mercury.Mass);

    println!("Rayon Orbital : {} m", radius);
    println!("Inclinaison Orbitale : {} degrés", inclination);

}
