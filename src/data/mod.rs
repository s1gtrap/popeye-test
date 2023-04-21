use std::collections::HashMap;
use std::fmt;

use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct Muscle {
    #[serde(skip_deserializing)]
    pub id: &'static str,
    pub name: &'static str,
    pub short_name: &'static str,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum Equipment {
    Barbell,
    BarbellBench,
    Dumbbell,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Exercise {
    #[serde(skip_deserializing)]
    pub id: &'static str,
    pub name: &'static str,
    pub muscle: &'static str,
    pub equipment: Equipment,
}

impl fmt::Debug for Exercise {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

const MUSCLES_JSON: &str = include_str!("muscles.json");
const EXERCISES_JSON: &str = include_str!("exercises.json");

lazy_static! {
    pub static ref MUSCLES: HashMap<&'static str, Muscle> =
        serde_json::from_str::<HashMap<_, Muscle>>(&MUSCLES_JSON)
            .unwrap()
            .into_iter()
            .map(|(id, body)| (
                id,
                Muscle {
                    id,
                    name: body.name,
                    short_name: body.short_name,
                }
            ))
            .collect();
    pub static ref EXERCISES: HashMap<&'static str, Exercise> =
        serde_json::from_str::<HashMap<_, Exercise>>(&EXERCISES_JSON)
            .unwrap()
            .into_iter()
            .map(|(id, body)| (
                id,
                Exercise {
                    id,
                    name: body.name,
                    muscle: body.muscle,
                    equipment: body.equipment,
                }
            ))
            .collect();
}

#[test]
fn test_muscles() {
    assert!(!MUSCLES.is_empty());
}
