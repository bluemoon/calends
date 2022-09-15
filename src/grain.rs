use crate::RelativeDuration;

pub enum Grain {
    Day,
    Week,
    Month,
    Quarter,
    Half,
    Year,
    Lustrum,
    Decade,
    Century,
}

impl Grain {
    pub fn into_duration(&self) -> RelativeDuration {
        match self {
            Grain::Day => RelativeDuration::days(1),
            Grain::Week => RelativeDuration::days(7),
            Grain::Month => RelativeDuration::months(1),
            Grain::Quarter => RelativeDuration::months(3),
            Grain::Half => RelativeDuration::months(6),
            Grain::Year => RelativeDuration::months(12),
            Grain::Lustrum => todo!(),
            Grain::Decade => todo!(),
            Grain::Century => todo!(),
        }
    }
}
