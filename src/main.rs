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

        // 0 - 5000 , cannot conclude anything
    }
    if let Some(planet) = &character.planet {
        match planet.trim().to_lowercase().as_str() {
            "earth" => return RINGS,
            "vogsphere" | "betelgeuse" => {
                return HITCH_HICKER;
            }
            "asgard" => return MARVEL,
            "kashyyyk" | "endor" => return STAR_WARS,
            other => {
                print!("some other planet:{other}");
            }
        }

        // conclusive
        panic!("unreachable");
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
            "extra_arms" | "extra_head" | "green" => Some(HITCH_HICKER),
            "hairy" => Some(STAR_WARS),
            "pointy_ears" => Some(RINGS),
            _ => None,
        }) {
            return universe;
        }

        if ref_to_norm.contains(&"blonde") && ref_to_norm.contains(&"tall") {
            return MARVEL;
        }

        if ref_to_norm.contains(&"short") && ref_to_norm.contains(&"bulky") {
            return RINGS;
        }

        // if only blonde || tall || short || bulcky -> unknown
    }

    // already filtered by planet, only age<5000 and
    // blonde || tall || short || bulcky traits remaining
    if let Some(known_type) = character.isHumanoid {
        match known_type {
            true => {
                if let Some(traits) = maybe_traits {
                    if traits.contains(&"tall".to_string()) {
                        return MARVEL;
                    }

                    if traits.contains(&"bulky".to_string())
                        || traits.contains(&"short".to_string())
                    {
                        return RINGS;
                    }

                    // if its blonde(and respectivly humanoid) and under 5000
                    // THEN it can be both elf and asgardian, but there are no other
                    // remaining metrics to judge by so ambiguous
                    return AMBIGUOUS;
                }
                // has no traits
                // cannot find from age < 5000 and humanity alone
                return AMBIGUOUS;
            }
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

                    // there is no blonde non-human
                }
                // any age(if any) is ambiguous
                // cannot be over 400, no non-humanoid has such lifespan
                // anything over 200 can be either from hitch or star wars
                return AMBIGUOUS;
            }
        }
    }
    // if no information is provided then its obviously ambiguous
    return AMBIGUOUS;
}

// classification related things
enum Universe {
    STAR_WARS,
    HITCH_HICKER,
    RINGS,
    MARVEL,
    AMBIGUOUS,
}

fn main() {
    // read the test-input.json
    let file_data = fs::read_to_string("resources/test-input.json").expect("failed to read file");
    let data: Data = serde_json::from_str(file_data.as_str()).expect("failed to parse json");

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
            AMBIGUOUS => todo!("handle case when it fails"),
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
