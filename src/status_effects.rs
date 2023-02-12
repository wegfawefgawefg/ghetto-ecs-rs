pub enum StatusEffectType {
    Poison,
    Burning,
    Stun,
    Slow,
}

pub struct StatusEffect {
    pub type_: StatusEffectType,
    pub duration: f32,
}
