use crate::model::equipement::Equipement;
use crate::model::skills::Skill;

pub struct Character {
    pub name: String,
    pub hp: i32,
    pub mana: i32,
    pub armor: i32,
    pub damage: i32,
    pub equipement: Equipement,
    pub skills: Vec<Skill>,
}

pub fn clad() -> Character {
    Character {
        name: "Clad".to_string(),
        hp: 100,
        mana: 30,
        armor: 10,
        damage: 30,
        equipement: Equipement {
            weapon: None,
            helmet: None,
            chest: None,
            legs: None,
            shoes: None,
        },
        skills: vec![],
    }
}