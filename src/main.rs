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

        //impossible à trigger on a energie infinie avec l'étoile
        if panel.energy_level < 5.{
            println!("energy level too low")
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

            if panel.temperature > 1500. {
                println!("{:?}", panel);
                println!("panel temperature too hight");
                println!("begin automatique correction");

                let ancient_coordinate = panel.clone();

                while panel.temperature > 1000.{
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

            }
        }
}
}



fn main() {
    const NUM_PANELS: usize = 10;
    const INITIAL_TEMPERATURE: f64 = -270.424;
    const DANGER_TEMPERATURE: f64 = 1668.; // température de fusion du titane

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
        let temperature = rng.gen_range(1000.0..1700.0);
        let energy_level = rng.gen_range(70.0..100.0);
        let connectivity = rng.gen_range(80..100);

        let solar_panel = SolarPanel::new(position, temperature, energy_level, connectivity);

        solar_panels.push(solar_panel);
    }


    for _ in 0..100 {
        rearrange_panels_hyperion(&mut solar_panels);
    }

    for panel in solar_panels{
        println!("{:?}", panel);
    }

}
