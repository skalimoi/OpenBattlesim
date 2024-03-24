use crate::token::{ActionToken, BodyToken, CommonToken, LogicToken, OptionsToken};

pub enum DataType {
    Actor,
    Material,
    Item
}

pub struct Actor {
    common: Vec<CommonToken>,
    body: Vec<BodyToken>,
    action: Vec<ActionToken>,
    logic: Vec<LogicToken>,
    options: Vec<OptionsToken>
}