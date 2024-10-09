#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

// allien related things
#[derive(Debug, Serialize, Deserialize)]
struct Data {
    data: Vec<Character>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Character {
    id: u32,
    isHumanoid: Option<bool>,
    planet: Option<String>,
    age: Option<u32>,
    traits: Option<Vec<String>>,
}

fn process(character: &Character) -> Universe {
    use Universe::*;

    if let Some(age) = character.age {
        if age > 5000 {
            return RINGS;
        }

        if age > 400 || age <= 5000 {
            return MARVEL;
        }

        if age > 200 || age <= 400 {
            return STAR_WARS;
        }

        // 0 - 200 , cannot conclude anything
    }

    if let Some(planet) = &character.planet {
        match planet.trim().to_lowercase().as_str() {
            "earth" => return RINGS,
            "vogsphere" | "betelguese" => return HITCH_HICKER,
            "asgard" => return MARVEL,
            "kashyyyk" | "endor" => return STAR_WARS,
            other => {
                print!("some other planet:{other}");
            }
        }

        // conclusive
    }

    let mut maybe_traits: Option<Vec<String>> = None;

    if let Some(traits) = &character.traits {
        // normalising traits
        maybe_traits = Some(
            traits
                .iter()
                .map(|_trait| _trait.trim().to_lowercase())
                .collect(),
        );
    }

    if let Some(traits) = &maybe_traits {
        // filter by traits

        let ref_to_norm: Vec<&str> = traits.iter().map(|x| x.as_str()).collect();

        if let Some(universe) = ref_to_norm.iter().find_map(|_trait| match *_trait {
            "extra _arms" | "extra_head" | "green" => Some(HITCH_HICKER),
            "hairy" => Some(STAR_WARS),
            "pointy_ears" => Some(RINGS),
            _ => None,
        }) {
            return universe;
        }

        if ref_to_norm.contains(&"blonde") || ref_to_norm.contains(&"tall") {
            return MARVEL;
        }

        if ref_to_norm.contains(&"short") || ref_to_norm.contains(&"bulky") {
            return RINGS;
        }

        // if only blonde / tall / short / bulcky -> unknown
    }

    if let Some(known_type) = character.isHumanoid {
        match known_type {
            true => {
                if let Some(traits) = maybe_traits {
                    if traits.contains(&"tall".to_string()) {
                        return MARVEL;
                    }

                    if traits.contains(&"bulcky".to_string()) {
                        if traits.contains(&"short".to_string()) {
                            return RINGS;
                        }

                        if let Some(age) = character.age {
                            if age < 200 {
                                return RINGS;
                            }

                            panic!(" cannot be bulky, over 200 and humanoid");
                        }
                    }

                    // any other trait is covered, blonde is either already filtered
                    // or undefined
                }
                //if let Some(age) = character.age {
                //  if age > 100
                //      not hitch, has to be rings or marvel
                //      doesnt have planet nor any traits, cannot conlude
                //  if age < 100
                //      can be anything, cannot conclude
                return UNKNOWN;
            } // is one of: marvel, hitch(covered by traits and, ring
            false => {
                if let Some(traits) = maybe_traits {
                    if traits.contains(&"tall".to_string()) // dont format
                    || traits.contains(&"short".to_string())
                    {
                        return STAR_WARS;
                    }

                    if traits.contains(&"bulky".to_string()) {
                        return HITCH_HICKER;
                    }
                }

                if let Some(_) = character.age {
                    return UNKNOWN;
                    // cannot find from age < 200 and non-humanoid alone
                }
            }
        }
    }

    return UNKNOWN;
}

//fn try_

// classification related things
#[allow(dead_code)]
enum Universe {
    STAR_WARS,
    HITCH_HICKER,
    RINGS,
    MARVEL,
    UNKNOWN,
}

#[allow(dead_code)]
struct Race_class {
    name: String,
    isHumanoid: bool,
    planet: String,
    age: Option<u32>,
    traits: Vec<String>,
}

#[allow(dead_code)]
struct Universe_class {
    universe: Universe,
    Races: Vec<Race_class>,
}

#[allow(dead_code)]
fn setup_classifications() -> Vec<Universe_class> {
    use Universe::*;

    vec![
        Universe_class {
            universe: STAR_WARS,
            Races: vec![
                Race_class {
                    name: "Wookie".to_string(),
                    isHumanoid: false,
                    planet: "Kashyyyk".to_string(),
                    age: Some(400),
                    traits: vec!["HAIRY".to_string(), "TALL".to_string()],
                },
                Race_class {
                    name: "Ewok".to_string(),
                    isHumanoid: false,
                    planet: "Endor".to_string(),
                    age: Some(60),
                    traits: vec!["SHORT".to_string(), "HAIRY".to_string()],
                },
            ],
        },
        Universe_class {
            universe: MARVEL,
            Races: vec![Race_class {
                name: "Asgardian".to_string(),
                isHumanoid: true,
                planet: "Asgard".to_string(),
                age: Some(5000),
                traits: vec!["BLONDE".to_string(), "TALL".to_string()],
            }],
        },
        Universe_class {
            universe: HITCH_HICKER,
            Races: vec![
                Race_class {
                    name: "Betelgeusian".to_string(),
                    isHumanoid: true,
                    planet: "Betelgeuse".to_string(),
                    age: Some(100),
                    traits: vec!["EXTRA_ARMS".to_string(), "EXTRA_HEAD".to_string()],
                },
                Race_class {
                    name: "Vogons".to_string(),
                    isHumanoid: false,
                    planet: "Vogsphere".to_string(),
                    age: Some(200),
                    traits: vec!["GREEN".to_string(), "BULKY".to_string()],
                },
            ],
        },
        Universe_class {
            universe: RINGS,
            Races: vec![
                Race_class {
                    name: "Ef".to_string(),
                    isHumanoid: true,
                    planet: "Earth".to_string(),
                    age: None,
                    traits: vec!["BLONDE".to_string(), "POINTY_EARS".to_string()],
                },
                Race_class {
                    name: "Dwarf".to_string(),
                    isHumanoid: true,
                    planet: "Earth".to_string(),
                    age: Some(200),
                    traits: vec!["SHORT".to_string(), "BULKY".to_string()],
                },
            ],
        },
    ]
}

fn main() {
    // read the test-input.json
    let file_data = fs::read_to_string("resources/input.json").expect("failed to read file");
    println!("{file_data}");
    let data: Data = serde_json::from_str(file_data.as_str()).expect("failed to parse json");

    println!("json data: {data:?}");
    let mut star_wars: Vec<Character> = vec![];
    let mut hitch_hicker: Vec<Character> = vec![];
    let mut rings: Vec<Character> = vec![];
    let mut marvel: Vec<Character> = vec![];

    // logic for choosing which universe it's from
    for entry in data.data {
        use Universe::*;
        match process(&entry) {
            STAR_WARS => star_wars.push(entry),
            HITCH_HICKER => hitch_hicker.push(entry),
            RINGS => rings.push(entry),
            MARVEL => marvel.push(entry),
            UNKNOWN => todo!("handle case when it fails"),
        }
    }

    // write each string as a .json in resources/output/_.json
    fs::write(
        "output/star-wars.json",
        serde_json::to_string(&star_wars).expect("failed to deserialize star wars"),
    )
    .expect("failed to write to file star_wars.json");

    fs::write(
        "output/hitch_hicker.json",
        serde_json::to_string(&hitch_hicker).expect("failed to deserialize hitch_hicker"),
    )
    .expect("failed to write to file hitch_hicker.json");

    fs::write(
        "output/rings.json",
        serde_json::to_string(&rings).expect("failed to deserialize rings"),
    )
    .expect("failed to write to file rings.json");

    fs::write(
        "output/marvel.json",
        serde_json::to_string(&marvel).expect("failed to deserialize marvel"),
    )
    .expect("failed to write to file marvel.json");
}
