mod positionnement;

extern crate rand;
extern crate nalgebra as na;

use rand::Rng;
use std::f64::consts::PI;
use na::{Point3, Vector3};

// Structure pour représenter un point en trois dimensions
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
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


impl Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}

// Structure pour représenter un panneau solaire
#[derive(Debug, Copy, Clone)]
pub struct SolarPanel {
    position: Point,
    temperature: f64,
    energy_level: f64,
    connectivity: i32,
    thruster: f64,
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

// Structure pour représenter un corps céleste (planète)
#[derive(Debug, Copy, Clone)]
struct CelestialBody {
    position: Point,
    velocity: Vector3<f64>,
}

impl CelestialBody {
    fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) -> CelestialBody {
        CelestialBody {
            position: Point::new(x, y, z),
            velocity: Vector3::new(vx, vy, vz),
        }
    }

    fn update_position(&mut self, dt: f64) {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        self.position.z += self.velocity.z * dt;
    }
}

// Fonction pour réarranger les panneaux en utilisant l'algorithme de recuit simulé.
fn rearrange_panels_hyperion(solar_swarm: &mut SolarSwarm, celestial_body: &mut CelestialBody) {
    let mut rng = rand::thread_rng();

    let cloned_solar = solar_swarm.solar_panels.clone();

    for panel in solar_swarm.solar_panels.iter_mut() {
        // Calculer la connectivité en fonction des panneaux proches
        let close_neighbors = cloned_solar.iter()
            .filter(|other_panel| panel.position.distance(&other_panel.position) < 20.0)
            .count() as i32;

        panel.connectivity = close_neighbors;

        // Copier les panneaux solaires pour le calcul de l'énergie
        let current_energy = calculate_energy(&cloned_solar);

        // Vérification de l'énergie minimale
        if panel.energy_level < 5. {
            println!("Energy level too low");
        }

        for _ in 0..100 {
            // Générer un déplacement aléatoire
            let dx = rng.gen_range(-panel.thruster..panel.thruster);
            let dy = rng.gen_range(-panel.thruster..panel.thruster);
            let dz = rng.gen_range(-panel.thruster..panel.thruster);

            // Appliquer le déplacement
            panel.position.x += dx;
            panel.position.y += dy;
            panel.position.z += dz;

            // Vérification de la distance de sécurité par rapport à la planète
            let new_distance_to_planet = panel.position.distance(&celestial_body.position);
            if new_distance_to_planet < 6.0 {
                // Reculer le panneau solaire en cas de collision avec la planète
                panel.position.x -= dx;
                panel.position.y -= dy;
                panel.position.z -= dz;

                println!("Collision with planet warning");
            }

            // Copier les panneaux solaires pour le calcul de l'énergie après le déplacement
            let new_energy = calculate_energy(&cloned_solar);

            // Vérification de la distance entre les panneaux ( à modifier pas optimal )
            let min_distance = 2.0;
            for other_panel in &cloned_solar {
                while other_panel.position.distance(&panel.position) < min_distance {
                    println!("Collision risk detected");
                    // Éloigner les panneaux si la distance est inférieure à 2
                    panel.position.x += dx * 2.0;
                    panel.position.y += dy * 2.0;
                    panel.position.z += dz * 2.0;
                }
            }

            let delta_energy = new_energy - current_energy;

            // Accepter ou rejeter le déplacement en fonction de l'énergie et de la température
            if delta_energy > 0.0 && rng.gen_range(0.0..1.0) > (-delta_energy / panel.temperature).exp() {
                panel.position.x -= dx;
                panel.position.y -= dy;
                panel.position.z -= dz;
            }

            // Gérer la surchauffe automatiquement
            if panel.temperature > 1500. {
                println!("Panel temperature too high");
                println!("Begin automatic correction");

                let ancient_coordinate = panel.clone();

                while panel.temperature > 1000. {
                    // Déplacer le panneau loin pour dissiper la chaleur
                    panel.position.x += 10.0;
                    panel.position.y += 10.0;
                    panel.position.z += 10.0;

                    // Ajouter des mesures pour surveiller la sonde thermique et ajuster la température
                    panel.temperature -= 15.0;
                }

                // Revenir à la position d'origine, mais rester plus loin pour éviter une autre surchauffe
                panel.position.x = ancient_coordinate.position.x - 5.0;
                panel.position.y = ancient_coordinate.position.y - 5.0;
                panel.position.z = ancient_coordinate.position.z - 5.0;

                println!("Alert ended");
            }
        }
    }

    // Mettre à jour la position de la planète
    celestial_body.update_position(1.0); // Changer la valeur de dt en fonction de la simulation
}

// Fonction pour calculer l'énergie totale des panneaux solaires
fn calculate_energy(solar_panels: &[SolarPanel]) -> f64 {
    let mut energy = 0.0;
    for i in 0..solar_panels.len() {
        for j in i + 1..solar_panels.len() {
            energy += 1.0 / solar_panels[i].position.distance(&solar_panels[j].position);
        }
    }
    energy
}

fn main() {
    const NUM_PANELS: usize = 10; // Nombre de panneaux solaires souhaité
    const DANGER_TEMPERATURE: f64 = 1668.0; // Température de fusion du titane
    const STAR_DIAMETER: f64 = 4.0; // Taille de l'étoile autour de laquelle on gravite

    let mut solar_panels = Vec::new();
    let mut rng = rand::thread_rng();
    let orbit_distance = 2.0;

    for _ in 0..NUM_PANELS {
        let theta = rng.gen_range(0.0..2.0 * PI);
        let phi = rng.gen_range(0.0..PI);
        let radius = orbit_distance + STAR_DIAMETER;

        let x = radius * theta.sin() * phi.cos();
        let y = radius * theta.sin() * phi.sin();
        let z = radius * theta.cos();

        let position = Point::new(x, y, z);
        let temperature = rng.gen_range(1300.0..1700.0);
        let energy_level = rng.gen_range(70.0..100.0);
        let connectivity = rng.gen_range(80..100);

        let solar_panel = SolarPanel::new(position, temperature, energy_level, connectivity, 1.0);

        solar_panels.push(solar_panel);
    }

    // Initialiser le système solaire avec une planète
    let mut solar_swarm = SolarSwarm::new("Hyperion", solar_panels);

    let mercury_distance = 57_910_000_000.0; // en mètres
    let mercury_orbital_velocity = 47_870.0; // en m/s
    let orbit_radius = mercury_distance + rng.gen_range(-1.0..1.0);
    const COMMON_ORBITAL_PERIOD: f64 = 58.6 * 24.0 * 3600.0;

    let mut mercury = CelestialBody::new(mercury_distance, 0.0, 0.0, 0.0, mercury_orbital_velocity, 0.0);


    // Simuler les déplacements des panneaux solaires et de la planète
    for _ in 0..10 {
        rearrange_panels_hyperion(&mut solar_swarm, &mut mercury);
    }



    // Définir la période orbitale commune (en secondes)


    let required_orbital_velocity = 2.0 * PI * orbit_radius / COMMON_ORBITAL_PERIOD;

    // Ajuster la vitesse initiale des panneaux solaires
    for panel in &mut solar_swarm.solar_panels {
        panel.thruster = required_orbital_velocity;
    }

    // Afficher les panneaux solaires après simulation
    for panel in &solar_swarm.solar_panels {
        println!("{:?}", panel);
    }
}
