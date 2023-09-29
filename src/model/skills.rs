pub struct Skill {
    pub name: String,
    pub damage: i32,
    pub range: i32,
    pub width: i32,
    pub action: Box<dyn Fn()>,
}