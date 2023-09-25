use crate::climate::HumidDry::Dry;
use crate::climate::HumidDry::Humid;
use crate::climate::HumidDry::None;
use std::ops::Range;

const COLD_SPRING: Range<f32> = 0.0..15.0;
const COLD_WINTER: Range<f32> = -25.0..0.0;
const COLD_FALL: Range<f32> = -10.0..5.0;
const COLD_SUMMER: Range<f32> = 5.0..17.0;

const TEMPERATE_SPRING: Range<f32> = 15.0..20.0;
const TEMPERATE_WINTER: Range<f32> = 0.0..11.0;
const TEMPERATE_FALL: Range<f32> = 7.0..15.0;
const TEMPERATE_SUMMER: Range<f32> = 20.0..27.0;

const WARM_SPRING: Range<f32> = 18.0..25.0;
const WARM_WINTER: Range<f32> = 5.0..15.0;
const WARM_FALL: Range<f32> = 10.0..20.0;
const WARM_SUMMER: Range<f32> = 25.0..33.0;

const HOT_SPRING: Range<f32> = 25.0..35.0;
const HOT_WINTER: Range<f32> = 15.0..22.0;
const HOT_FALL: Range<f32> = 17.0..25.0;
const HOT_SUMMER: Range<f32> = 30.0..45.0;

pub const EQUATOR_TEMP_RANGE: Range<f32> = 0.0..3.0;
pub const TEMPERATE_TEMP_RANGE: Range<f32> = 7.0..10.0;
pub const CONTINENTAL_POLAR_TEMP_RANGE: Range<f32> = 12.0..25.0;

#[derive(Copy, Clone)]
pub enum HumidDry {
    Humid,
    Dry,
    None,
}

#[derive(Clone)]
pub struct Climate<'a> {
    pub name: &'a str,
    pub general_type: char,
    pub second_type: char,
    pub third_type: char,
    pub spring: Range<f32>,
    pub winter: (HumidDry, Range<f32>),
    pub fall: Range<f32>,
    pub summer: (HumidDry, Range<f32>),
    pub diurnal_range: Range<f32>
}

// TODO: evapotranspiration? Precipitation type?

pub const KOPPEN_AF_AM: Climate = Climate {
    name: "Tropical rainforest / monsoon",
    general_type: 'A',
    second_type: 'F',
    third_type: '_',
    spring: HOT_SPRING,
    winter: (None, HOT_WINTER),
    fall: HOT_FALL,
    summer: (Humid, HOT_SUMMER),
    diurnal_range: EQUATOR_TEMP_RANGE
};

pub const KOPPEN_AS: Climate = Climate {
    name: "Tropical dry savanna",
    general_type: 'A',
    second_type: 'F',
    third_type: '_',
    spring: HOT_SPRING,
    winter: (None, HOT_WINTER),
    fall: HOT_FALL,
    summer: (None, HOT_SUMMER),
    diurnal_range: EQUATOR_TEMP_RANGE
};

pub const KOPPEN_AW: Climate = Climate {
    name: "Tropical wet savanna",
    general_type: 'A',
    second_type: 'F',
    third_type: '_',
    spring: HOT_SPRING,
    winter: (Humid, HOT_WINTER),
    fall: HOT_FALL,
    summer: (None, HOT_SUMMER),
    diurnal_range: EQUATOR_TEMP_RANGE
};

pub const KOPPEN_BSH: Climate = Climate {
    name: "Hot steppe",
    general_type: 'B',
    second_type: 'S',
    third_type: 'H',
    spring: WARM_SPRING,
    winter: (Dry, WARM_WINTER),
    fall: TEMPERATE_FALL,
    summer: (None, HOT_SUMMER),
    diurnal_range: CONTINENTAL_POLAR_TEMP_RANGE
};

pub const KOPPEN_BSK: Climate = Climate {
    name: "Cold steppe",
    general_type: 'B',
    second_type: 'S',
    third_type: 'H',
    spring: COLD_SPRING,
    winter: (Dry, COLD_WINTER),
    fall: COLD_FALL,
    summer: (None, COLD_SUMMER),
    diurnal_range: CONTINENTAL_POLAR_TEMP_RANGE
};

pub const KOPPEN_BWH: Climate = Climate {
    name: "Hot desert",
    general_type: 'B',
    second_type: 'W',
    third_type: 'H',
    spring: HOT_SPRING,
    winter: (Dry, HOT_WINTER),
    fall: HOT_FALL,
    summer: (Dry, HOT_SUMMER),
    diurnal_range: TEMPERATE_TEMP_RANGE
};

pub const KOPPEN_BWK: Climate = Climate {
    name: "Cold desert",
    general_type: 'B',
    second_type: 'W',
    third_type: 'K',
    spring: TEMPERATE_SPRING,
    winter: (Dry, COLD_WINTER),
    fall: TEMPERATE_FALL,
    summer: (Dry, COLD_SUMMER),
    diurnal_range: TEMPERATE_TEMP_RANGE
};

pub const KOPPEN_CFA: Climate = Climate {
    name: "Humid subtropical",
    general_type: 'C',
    second_type: 'F',
    third_type: 'A',
    spring: TEMPERATE_SPRING,
    winter: (Humid, TEMPERATE_WINTER),
    fall: TEMPERATE_FALL,
    summer: (Humid, HOT_SUMMER),
    diurnal_range: TEMPERATE_TEMP_RANGE
};

pub const KOPPEN_CFB: Climate = Climate {
    name: "Temperate oceanic",
    general_type: 'C',
    second_type: 'F',
    third_type: 'B',
    spring: TEMPERATE_SPRING,
    winter: (Humid, TEMPERATE_WINTER),
    fall: TEMPERATE_FALL,
    summer: (Humid, WARM_SUMMER),
    diurnal_range: TEMPERATE_TEMP_RANGE
};

pub const KOPPEN_CFC: Climate = Climate {
    name: "Subpolar oceanic",
    general_type: 'C',
    second_type: 'F',
    third_type: 'C',
    spring: TEMPERATE_SPRING,
    winter: (Humid, COLD_WINTER),
    fall: COLD_FALL,
    summer: (Humid, TEMPERATE_WINTER),
    diurnal_range: TEMPERATE_TEMP_RANGE
};

pub const KOPPEN_CSA: Climate = Climate {
    name: "Hot-summer mediterranean",
    general_type: 'C',
    second_type: 'S',
    third_type: 'A',
    spring: WARM_SPRING,
    winter: (None, TEMPERATE_WINTER),
    fall: TEMPERATE_FALL,
    summer: (Dry, HOT_SUMMER),
    diurnal_range: TEMPERATE_TEMP_RANGE
};

pub const KOPPEN_CSB: Climate = Climate {
    name: "Warm-summer mediterranean",
    general_type: 'B',
    second_type: 'W',
    third_type: 'H',
    spring: WARM_SPRING,
    winter: (None, TEMPERATE_WINTER),
    fall: TEMPERATE_FALL,
    summer: (Dry, WARM_SUMMER),
    diurnal_range: TEMPERATE_TEMP_RANGE
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
    spring: WARM_SPRING,
    winter: (Dry, TEMPERATE_WINTER),
    fall: TEMPERATE_FALL,
    summer: (Humid, HOT_SUMMER),
    diurnal_range: TEMPERATE_TEMP_RANGE
};

pub const KOPPEN_CWB: Climate = Climate {
    name: "Subtropical highland",
    general_type: 'C',
    second_type: 'W',
    third_type: 'B',
    spring: WARM_SPRING,
    winter: (Dry, TEMPERATE_WINTER),
    fall: TEMPERATE_FALL,
    summer: (Humid, WARM_SUMMER),
    diurnal_range: CONTINENTAL_POLAR_TEMP_RANGE
};

pub const KOPPEN_CWC: Climate = Climate {
    name: "Cold subtropical highland",
    general_type: 'C',
    second_type: 'W',
    third_type: 'C',
    spring: TEMPERATE_SPRING,
    winter: (Dry, TEMPERATE_WINTER),
    fall: COLD_FALL,
    summer: (Humid, COLD_SUMMER),
    diurnal_range: CONTINENTAL_POLAR_TEMP_RANGE
};

pub const KOPPEN_DFA: Climate = Climate {
    name: "Hot humid continental",
    general_type: 'D',
    second_type: 'F',
    third_type: 'A',
    spring: TEMPERATE_SPRING,
    winter: (Humid, COLD_WINTER),
    fall: TEMPERATE_FALL,
    summer: (Humid, HOT_SUMMER),
    diurnal_range: CONTINENTAL_POLAR_TEMP_RANGE
};

pub const KOPPEN_DFB: Climate = Climate {
    name: "Warm humid continental",
    general_type: 'D',
    second_type: 'F',
    third_type: 'B',
    spring: TEMPERATE_SPRING,
    winter: (Humid, COLD_WINTER),
    fall: TEMPERATE_FALL,
    summer: (Humid, WARM_SUMMER),
    diurnal_range: TEMPERATE_TEMP_RANGE
};

pub const KOPPEN_DFC: Climate = Climate {
    name: "Subarctic",
    general_type: 'D',
    second_type: 'F',
    third_type: 'C',
    spring: TEMPERATE_SPRING,
    winter: (Humid, COLD_WINTER),
    fall: COLD_FALL,
    summer: (Humid, COLD_SUMMER),
    diurnal_range: CONTINENTAL_POLAR_TEMP_RANGE
};

pub const KOPPEN_DSA: Climate = Climate {
    name: "Hot continental",
    general_type: 'D',
    second_type: 'S',
    third_type: 'A',
    spring: HOT_SPRING,
    winter: (None, COLD_WINTER),
    fall: COLD_FALL,
    summer: (Dry, HOT_SUMMER),
    diurnal_range: CONTINENTAL_POLAR_TEMP_RANGE
};

pub const KOPPEN_DSB: Climate = Climate {
    name: "Warm continental",
    general_type: 'D',
    second_type: 'S',
    third_type: 'B',
    spring: WARM_SPRING,
    winter: (None, COLD_WINTER),
    fall: COLD_FALL,
    summer: (Dry, WARM_SUMMER),
    diurnal_range: CONTINENTAL_POLAR_TEMP_RANGE
};

pub const KOPPEN_DSC: Climate = Climate {
    name: "Dry subarctic",
    general_type: 'D',
    second_type: 'S',
    third_type: 'C',
    spring: COLD_SPRING,
    winter: (None, COLD_WINTER),
    fall: COLD_FALL,
    summer: (Dry, COLD_SUMMER),
    diurnal_range: CONTINENTAL_POLAR_TEMP_RANGE
};

pub const KOPPEN_ET: Climate = Climate {
    name: "Tundra",
    general_type: 'E',
    second_type: 'T',
    third_type: '_',
    spring: COLD_SPRING,
    winter: (Dry, COLD_WINTER),
    fall: COLD_FALL,
    summer: (Dry, COLD_SUMMER),
    diurnal_range: CONTINENTAL_POLAR_TEMP_RANGE
};

