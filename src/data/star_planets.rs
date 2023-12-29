use std::cell::UnsafeCell;
use std::rc::Rc;

use super::enums::{SpectrType, StarType};
use super::planet::Planet;
use super::random::DspRandom;
use super::star::Star;
use serde::Serialize;

pub fn serialize_planets<S>(
    planets: &UnsafeCell<Vec<Rc<Planet<'_>>>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    unsafe { &*planets.get() }.serialize(serializer)
}

#[derive(Debug, Serialize)]
pub struct StarWithPlanets<'a> {
    #[serde(flatten)]
    pub star: Rc<Star<'a>>,
    #[serde(serialize_with = "serialize_planets")]
    planets: UnsafeCell<Vec<Rc<Planet<'a>>>>,
}

impl<'a> StarWithPlanets<'a> {
    pub fn new(star: Rc<Star<'a>>) -> Self {
        Self {
            star,
            planets: UnsafeCell::new(vec![]),
        }
    }

    pub fn get_planets(&self) -> &Vec<Rc<Planet<'a>>> {
        let planets = unsafe { &mut *self.planets.get() };
        if !planets.is_empty() {
            return planets;
        }
        let mut rand2 = DspRandom::new(self.star.planets_seed);
        let num1 = rand2.next_f64();
        let num2 = rand2.next_f64();
        let num3 = if rand2.next_f64() > 0.5 { 1 } else { 0 };
        rand2.next_f64();
        rand2.next_f64();
        rand2.next_f64();
        rand2.next_f64();

        let mut make_planet = |index: i32, orbit_index: i32, gas_giant: bool| -> Rc<Planet> {
            let info_seed = rand2.next_seed();
            let gen_seed = rand2.next_seed();
            Rc::new(Planet::new(
                self.star.clone(),
                index,
                orbit_index,
                gas_giant,
                info_seed,
                gen_seed,
            ))
        };

        let star_type = &self.star.star_type;

        if star_type == &StarType::BlackHole || star_type == &StarType::NeutronStar {
            planets.push(make_planet(0, 3, false));
        } else if star_type == &StarType::WhiteDwarf {
            if num1 < 0.7 {
                planets.push(make_planet(0, 3, false));
            } else if num2 < 0.3 {
                planets.push(make_planet(0, 3, false));
                planets.push(make_planet(1, 4, false));
            } else {
                planets.push(make_planet(0, 4, true));
                planets.push(make_planet(1, 1, false));
                let planet1 = &planets[0];
                let planet2 = &planets[1];
                planet2.orbit_around.replace(Some(planet1.clone()));
            }
        } else if star_type == &StarType::GiantStar {
            if num1 < 0.3 {
                planets.push(make_planet(0, 2 + num3, false));
            } else if num1 < 0.8 {
                if num2 < 0.25 {
                    planets.push(make_planet(0, 2 + num3, false));
                    planets.push(make_planet(1, 3 + num3, false));
                } else {
                    planets.push(make_planet(0, 3, true));
                    planets.push(make_planet(1, 1, false));
                    let planet1 = &planets[0];
                    let planet2 = &planets[1];
                    planet2.orbit_around.replace(Some(planet1.clone()));
                }
            } else {
                if num2 < 0.15 {
                    planets.push(make_planet(0, 2 + num3, false));
                    planets.push(make_planet(1, 3 + num3, false));
                    planets.push(make_planet(2, 4 + num3, false));
                } else if num2 < 0.75 {
                    planets.push(make_planet(0, 2 + num3, false));
                    planets.push(make_planet(1, 4, true));
                    planets.push(make_planet(2, 1, false));
                    let planet2 = &planets[1];
                    let planet3 = &planets[2];
                    planet3.orbit_around.replace(Some(planet2.clone()));
                } else {
                    planets.push(make_planet(0, 3 + num3, true));
                    planets.push(make_planet(1, 1, false));
                    planets.push(make_planet(2, 2, false));
                    let planet1 = &planets[0];
                    let planet2 = &planets[1];
                    let planet3 = &planets[2];
                    planet2.orbit_around.replace(Some(planet1.clone()));
                    planet3.orbit_around.replace(Some(planet1.clone()));
                }
            }
        } else {
            let (planet_count, p_gas): (i32, [f64; 6]) = if self.star.is_birth() {
                (4, P_GASES[0])
            } else {
                match self.star.get_spectr() {
                    SpectrType::M => {
                        let planet_count = if num1 >= 0.8 {
                            4
                        } else if num1 >= 0.3 {
                            3
                        } else if num1 >= 0.1 {
                            2
                        } else {
                            1
                        };
                        (
                            planet_count,
                            if planet_count <= 3 {
                                P_GASES[1]
                            } else {
                                P_GASES[2]
                            },
                        )
                    }
                    SpectrType::K => {
                        let planet_count = if num1 >= 0.95 {
                            5
                        } else if num1 >= 0.7 {
                            4
                        } else if num1 >= 0.2 {
                            3
                        } else if num1 >= 0.1 {
                            2
                        } else {
                            1
                        };
                        (
                            planet_count,
                            if planet_count <= 3 {
                                P_GASES[3]
                            } else {
                                P_GASES[4]
                            },
                        )
                    }
                    SpectrType::G => {
                        let planet_count = if num1 >= 0.9 {
                            5
                        } else if num1 >= 0.4 {
                            4
                        } else {
                            3
                        };
                        (
                            planet_count,
                            if planet_count <= 3 {
                                P_GASES[3]
                            } else {
                                P_GASES[5]
                            },
                        )
                    }
                    SpectrType::F => {
                        let planet_count = if num1 >= 0.8 {
                            5
                        } else if num1 >= 0.35 {
                            4
                        } else {
                            3
                        };
                        (
                            planet_count,
                            if planet_count <= 3 {
                                P_GASES[1]
                            } else {
                                P_GASES[6]
                            },
                        )
                    }
                    SpectrType::A => {
                        let planet_count = if num1 >= 0.75 {
                            5
                        } else if num1 >= 0.3 {
                            4
                        } else {
                            3
                        };
                        (
                            planet_count,
                            if planet_count <= 3 {
                                P_GASES[1]
                            } else {
                                P_GASES[7]
                            },
                        )
                    }
                    SpectrType::B => {
                        let planet_count = if num1 >= 0.75 {
                            6
                        } else if num1 >= 0.3 {
                            5
                        } else {
                            4
                        };
                        (
                            planet_count,
                            if planet_count <= 3 {
                                P_GASES[1]
                            } else {
                                P_GASES[8]
                            },
                        )
                    }
                    SpectrType::O => {
                        let planet_count = if num1 >= 0.5 { 6 } else { 5 };
                        (planet_count, P_GASES[9])
                    }
                    _ => (1, P_GASES[0]),
                }
            };
            let mut num8 = 0;
            let mut num9 = 0;
            let mut orbit_around: i32 = 0;
            let mut num10 = 1;
            for index in 0..planet_count {
                let info_seed = rand2.next_seed();
                let gen_seed = rand2.next_seed();
                let num11 = rand2.next_f64();
                let num12 = rand2.next_f64();
                let mut gas_giant = false;
                if orbit_around == 0 {
                    num8 += 1;
                    if index < planet_count - 1 && num11 < p_gas[index as usize] {
                        gas_giant = true;
                        if num10 < 3 {
                            num10 = 3;
                        }
                    }
                    let mut broke_from_loop = false;
                    while !self.star.is_birth() || num10 != 3 {
                        let num13 = planet_count - index;
                        let num14 = 9 - (num10 as i32);
                        if num14 > num13 {
                            let a = (num13 as f32) / (num14 as f32);
                            let a2 = if num10 <= 3 { 0.15_f32 } else { 0.45_f32 };
                            let num15 = a + (1.0 - a) * a2 + 0.01;
                            if rand2.next_f64() < num15 as f64 {
                                broke_from_loop = true;
                                break;
                            }
                        } else {
                            broke_from_loop = true;
                            break;
                        }
                        num10 += 1;
                    }
                    if !broke_from_loop {
                        gas_giant = true;
                    }
                } else {
                    num9 += 1;
                }
                let planet = Planet::new(
                    self.star.clone(),
                    index,
                    if orbit_around == 0 { num10 } else { num9 },
                    gas_giant,
                    info_seed,
                    gen_seed,
                );
                if orbit_around > 0 {
                    planet
                        .orbit_around
                        .replace(Some(planets[orbit_around as usize - 1].clone()));
                }
                num10 += 1;
                if gas_giant {
                    orbit_around = num8;
                    num9 = 0;
                }
                if num9 >= 1 && num12 < 0.8 {
                    orbit_around = 0;
                    num9 = 0;
                }
                planets.push(Rc::new(planet));
            }
        }

        planets
    }
}

const P_GASES: [[f64; 6]; 10] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],     // birth
    [0.2, 0.2, 0.0, 0.0, 0.0, 0.0],     // M / F / A / B, n <= 3
    [0.0, 0.2, 0.3, 0.0, 0.0, 0.0],     // M, n >= 4
    [0.18, 0.18, 0.0, 0.0, 0.0, 0.0],   // K / G, n <= 3
    [0.0, 0.18, 0.28, 0.28, 0.0, 0.0],  // K, n >= 4
    [0.0, 0.2, 0.3, 0.3, 0.0, 0.0],     // G, n >= 4
    [0.0, 0.22, 0.31, 0.31, 0.0, 0.0],  // F, n >= 4
    [0.1, 0.28, 0.3, 0.35, 0.0, 0.0],   // A, n >= 4
    [0.1, 0.22, 0.28, 0.35, 0.35, 0.0], // B, n >= 4
    [0.1, 0.2, 0.25, 0.3, 0.32, 0.35],  // O
];