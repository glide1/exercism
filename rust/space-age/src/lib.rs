pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        (d.0 as f64 / 31557600f64) / Self::earth_year_ratio()
    }

    fn earth_year_ratio() -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn earth_year_ratio() -> f64 {
        0.2408467f64
    }
}
impl Planet for Venus {
    fn earth_year_ratio() -> f64 {
        0.61519726f64
    }
}
impl Planet for Earth {
    fn earth_year_ratio() -> f64 {
        1f64
    }
}
impl Planet for Mars {
    fn earth_year_ratio() -> f64 {
        1.8808158f64
    }
}
impl Planet for Jupiter {
    fn earth_year_ratio() -> f64 {
        11.862615f64
    }
}
impl Planet for Saturn {
    fn earth_year_ratio() -> f64 {
        29.447498f64
    }
}
impl Planet for Uranus {
    fn earth_year_ratio() -> f64 {
        84.016846f64
    }
}
impl Planet for Neptune {
    fn earth_year_ratio() -> f64 {
        164.79132f64
    }
}