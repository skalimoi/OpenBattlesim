use crate::climate::HotCold::Cold;
use crate::climate::HotCold::Warm;
use crate::climate::HotCold::{Hot, Temperate};
use crate::climate::HumidDry::Dry;
use crate::climate::HumidDry::Humid;
use crate::climate::HumidDry::None;
use std::ops::Range;

// TODO: CAMBIAR ENUMS QUE NO SIGNIFICAN NADA POR RANGES DE TEMPERATURA BASE PARA LA GENERACION

const COLD_SPRING: Range<f64> = 0.0..15.0;
const COLD_WINTER: Range<f64> = -25.0..0.0;
const COLD_FALL: Range<f64> = -10.0..5.0;
const COLD_SUMMER: Range<f64> = 5.0..17.0;

const TEMPERATE_SPRING: Range<f64> = 15.0..20.0;
const TEMPERATE_WINTER: Range<f64> = 0.0..11.0;
const TEMPERATE_FALL: Range<f64> = 7.0..15.0;
const TEMPERATE_SUMMER: Range<f64> = 20.0..27.0;

const WARM_SPRING: Range<f64> = 18.0..25.0;
const WARM_WINTER: Range<f64> = 5.0..15.0;
const WARM_FALL: Range<f64> = 10.0..20.0;
const WARM_SUMMER: Range<f64> = 25.0..33.0;

const HOT_SPRING: Range<f64> = 25.0..35.0;
const HOT_WINTER: Range<f64> = 15.0..22.0;
const HOT_FALL: Range<f64> = 17.0..25.0;
const HOT_SUMMER: Range<f64> = 30.0..45.0;

enum HumidDry {
    Humid,
    Dry,
    None,
}

enum HotCold {
    Hot,
    Cold,
    Warm,
    Temperate,
}

pub struct Climate<'a> {
    pub name: &'a str,
    pub general_type: char,
    pub second_type: char,
    pub third_type: char,

    winter: (HumidDry, HotCold),
    summer: (HumidDry, HotCold),
}

// TODO: evapotranspiration? Precipitation type?

pub const KOPPEN_AF_AM: Climate = Climate {
    name: "Tropical rainforest / monsoon",
    general_type: 'A',
    second_type: 'F',
    third_type: '_',
    winter: (None, Hot),
    summer: (Humid, Hot),
};

pub const KOPPEN_AS: Climate = Climate {
    name: "Tropical dry savanna",
    general_type: 'A',
    second_type: 'F',
    third_type: '_',
    winter: (None, Hot),
    summer: (None, Hot),
};

pub const KOPPEN_AW: Climate = Climate {
    name: "Tropical wet savanna",
    general_type: 'A',
    second_type: 'F',
    third_type: '_',
    winter: (Humid, Hot),
    summer: (None, Hot),
};

pub const KOPPEN_BSH: Climate = Climate {
    name: "Hot steppe",
    general_type: 'B',
    second_type: 'S',
    third_type: 'H',
    winter: (Dry, Warm),
    summer: (None, Hot),
};

pub const KOPPEN_BSK: Climate = Climate {
    name: "Cold steppe",
    general_type: 'B',
    second_type: 'S',
    third_type: 'H',
    winter: (Dry, Cold),
    summer: (None, Cold),
};

pub const KOPPEN_BWH: Climate = Climate {
    name: "Hot desert",
    general_type: 'B',
    second_type: 'W',
    third_type: 'H',
    winter: (Dry, Hot),
    summer: (Dry, Hot),
};

pub const KOPPEN_BWK: Climate = Climate {
    name: "Cold desert",
    general_type: 'B',
    second_type: 'W',
    third_type: 'K',
    winter: (Dry, Cold),
    summer: (Dry, Cold),
};

pub const KOPPEN_CFA: Climate = Climate {
    name: "Humid subtropical",
    general_type: 'C',
    second_type: 'F',
    third_type: 'A',
    winter: (Humid, Temperate),
    summer: (Humid, Hot),
};

pub const KOPPEN_CFB: Climate = Climate {
    name: "Temperate oceanic",
    general_type: 'C',
    second_type: 'F',
    third_type: 'B',
    winter: (Humid, Temperate),
    summer: (Humid, Warm),
};

pub const KOPPEN_CFC: Climate = Climate {
    name: "Subpolar oceanic",
    general_type: 'C',
    second_type: 'F',
    third_type: 'C',
    winter: (Humid, Cold),
    summer: (Humid, Temperate),
};

pub const KOPPEN_CSA: Climate = Climate {
    name: "Hot-summer mediterranean",
    general_type: 'C',
    second_type: 'S',
    third_type: 'A',
    winter: (None, Temperate),
    summer: (Dry, Hot),
};

pub const KOPPEN_CSB: Climate = Climate {
    name: "Warm-summer mediterranean",
    general_type: 'B',
    second_type: 'W',
    third_type: 'H',
    winter: (None, Temperate),
    summer: (Dry, Warm),
};

// pub const KOPPEN_CSC: Climate = Climate {
//     name: "Cool-summer mediterranean",
//     general_type: 'C',
//     second_type: 'S',
//     third_type: 'c',
//     winter: (None, Temperate),
//     summer: (Dry, Temperate),
// };

pub const KOPPEN_CWA: Climate = Climate {
    name: "Monsoon subtropical",
    general_type: 'C',
    second_type: 'W',
    third_type: 'A',
    winter: (Dry, Temperate),
    summer: (Humid, Hot),
};

pub const KOPPEN_CWB: Climate = Climate {
    name: "Subtropical highland",
    general_type: 'C',
    second_type: 'W',
    third_type: 'B',
    winter: (Dry, Temperate),
    summer: (Humid, Warm),
};

pub const KOPPEN_CWC: Climate = Climate {
    name: "Cold subtropical highland",
    general_type: 'C',
    second_type: 'W',
    third_type: 'C',
    winter: (Dry, Temperate),
    summer: (Humid, Cold),
};

pub const KOPPEN_DFA: Climate = Climate {
    name: "Hot humid continental",
    general_type: 'D',
    second_type: 'F',
    third_type: 'A',
    winter: (Humid, Cold),
    summer: (Humid, Hot),
};

pub const KOPPEN_DFB: Climate = Climate {
    name: "Warm humid continental",
    general_type: 'D',
    second_type: 'F',
    third_type: 'B',
    winter: (Humid, Cold),
    summer: (Humid, Warm),
};

pub const KOPPEN_DFC: Climate = Climate {
    name: "Subarctic",
    general_type: 'D',
    second_type: 'F',
    third_type: 'C',
    winter: (Humid, Cold),
    summer: (Humid, Cold),
};

pub const KOPPEN_DSA: Climate = Climate {
    name: "Hot continental",
    general_type: 'D',
    second_type: 'S',
    third_type: 'A',
    winter: (None, Cold),
    summer: (Dry, Hot),
};

pub const KOPPEN_DSB: Climate = Climate {
    name: "Warm continental",
    general_type: 'D',
    second_type: 'S',
    third_type: 'B',
    winter: (None, Cold),
    summer: (Dry, Warm),
};

pub const KOPPEN_DSC: Climate = Climate {
    name: "Dry subarctic",
    general_type: 'D',
    second_type: 'S',
    third_type: 'C',
    winter: (None, Cold),
    summer: (Dry, Cold),
};

pub const KOPPEN_ET: Climate = Climate {
    name: "Tundra",
    general_type: 'E',
    second_type: 'T',
    third_type: '_',
    winter: (Dry, Cold),
    summer: (Dry, Cold),
};

pub struct DecodedClimateData {
    spring: (u8, Range<f64>),
    winter: (u8, Range<f64>),
    fall: (u8, Range<f64>),
    summer: (u8, Range<f64>),
}

/// Outputs in this order: spring, winter, fall, summer.
pub fn data_from_climate(climate: Climate) {
    // none:0, dry: 1, humid: 2

    let c_type = String::from_iter(vec![
        climate.general_type,
        climate.second_type,
        climate.third_type,
    ]);

    let spring_temp_range: (u8, Range<f64>);
    let winter_temp_range: (u8, Range<f64>);
    let fall_temp_range: (u8, Range<f64>);
    let summer_temp_range: (u8, Range<f64>);

    // Temperature choosing
    match c_type.as_str() {
        "ET_" => {
            winter_temp_range = (1, COLD_WINTER);
            fall_temp_range = (0, COLD_FALL);
            summer_temp_range = (1, COLD_SUMMER);
            spring_temp_range = (0, COLD_SPRING);
        }
        "DSc" => {
            winter_temp_range = (1, COLD_WINTER);
            fall_temp_range = (0, COLD_FALL);
            summer_temp_range = (1, COLD_SUMMER);
            spring_temp_range = (0, COLD_SPRING);
        }
        _ => todo!(),
    }
}
