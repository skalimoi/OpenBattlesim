use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Range;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::{Display, EnumIter, EnumString}; // 0.17.1

pub trait Token: Debug {
}



#[derive(Debug, EnumIter, EnumString, Display)]
pub enum TCommon {
    TEST_ONE,
    TEST_TWO(u8, u8)
}
impl Token for TCommon {
}

#[derive(Debug, EnumIter, EnumString, Display)]
pub enum TBody {
    StrengthRange(Range<u16>)
}
impl Token for TBody {
}
#[derive(Debug, EnumIter, EnumString, Display)]
pub enum TAction {
    ACTION_TOKEN
}
impl Token for TAction {
}
#[derive(Debug, EnumIter, EnumString, Display)]
pub enum TLogic {
    LOGIC_TOKEN
}
impl Token for TLogic {
}
#[derive(Debug, EnumIter, EnumString, Display)]
pub enum TOption {
    OPTION_TOKEN
}
impl Token for TOption {
}