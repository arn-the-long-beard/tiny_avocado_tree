
pub enum Power {
    NoPower,
    BabyPower(u8),
    LowPower(u8),
    MediumPower(u8),
    HighPower(u8),
    MaxPower(u8),
}
impl Default for Power {
    fn default() -> Self {
        Power::NoPower
    }
}
impl Power {
    pub fn rank(nb: u8) -> Power {
        match nb {
            0 => Power::NoPower,
            1..=10 => Power::BabyPower(nb),
            11..=25 => Power::LowPower(nb),
            26..=100 => Power::MediumPower(nb),
            101..=200 => Power::HighPower(nb),
            201..=255 => Power::MaxPower(nb),
            _ => Power::NoPower,
        }
    }
    pub fn display(&self) -> &str {
        match &self {
            Power::NoPower => "Cannot live",
            Power::BabyPower(pw) => "I am a baby and I cry a lot",
            Power::LowPower(pw) => "I am so weak ",
            Power::MediumPower(pw) => "I am ok ",
            Power::HighPower(pw) => "I feel so good ",
            Power::MaxPower(pw) => "Mouahaha I am so OP",
        }
    }
    pub fn class(&self) -> &str {
        match &self {
            Power::NoPower => "no-power",
            Power::BabyPower(pw) => "baby-power",
            Power::LowPower(pw) => "low-power",
            Power::MediumPower(pw) => "medium-power",
            Power::HighPower(pw) => "high-power",
            Power::MaxPower(pw) => "max-power",
        }
    }
    pub fn units(&self) -> &u8 {
        match &self {
            Power::NoPower => &0,
            Power::BabyPower(pw) => pw,
            Power::LowPower(pw) => pw,
            Power::MediumPower(pw) => pw,
            Power::HighPower(pw) => pw,
            Power::MaxPower(pw) => pw,
        }
    }
}