// mod concentration;
// mod dimension;
// mod volume;

/// Value as in decimal * 100
pub type Percent = f32;
/// Value as in decimal * 1e6
pub type PartsPerMillion = f32;
/// pH value, 7 is neutral
pub type PH = f32;
/// Wort density
pub type SpecificGravity = f32;
/// International bitternes units
pub type Ibu = f32;
/// Standard reference method
///
/// [Reference](https://beermaverick.com/understanding-srm-and-lovibond-beer-color-calculations/)
pub type SRMColor = f32;
/// Carbonation
///
/// TODO: properly documents
pub type VolumesCO2 = f32;
/// Alcohol by volume in percent
pub type Abv = Percent;
/// Temperature
pub type Celsius = f32;
/// Volume
pub type Liters = f32;
/// Mass
pub type Kilograms = f32;
/// Time
pub type Minutes = f32;
/// Time
pub type Days = f32;
