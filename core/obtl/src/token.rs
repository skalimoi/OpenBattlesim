use std::ops::Range;

pub enum CommonToken {
    Name(String),
}
pub enum BodyToken {
    StrengthRange(Range<u16>)
}

pub enum ActionToken {
    CanSwim
}

pub enum LogicToken {
    OptionOneLogic
}

pub enum OptionsToken {
    MiscOption
}
