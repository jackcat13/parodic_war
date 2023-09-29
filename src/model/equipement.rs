use crate::model::chest::Chest;
use crate::model::helmet::Helmet;
use crate::model::legs::Legs;
use crate::model::shoes::Shoes;
use crate::model::weapon::Weapon;

pub struct Equipement {
    pub weapon: Option<Weapon>,
    pub helmet: Option<Helmet>,
    pub chest: Option<Chest>,
    pub legs: Option<Legs>,
    pub shoes: Option<Shoes>,
}