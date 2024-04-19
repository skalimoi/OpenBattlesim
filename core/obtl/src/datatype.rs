use std::fmt::{Debug, Formatter};
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use crate::datatype::GotoVariant::{After, Before};
use crate::datatype::GroupType::Common;
use crate::parser::{OBTLParser, Rule};
use crate::token::{TAction, TBody, TCommon, TLogic, Token, TOption};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::{Display, EnumIter};
use log::error;
use crate::token; // 0.17.1


#[derive(Debug)]
pub enum DataType {
    Actor,
    Material,
    Item
}

#[derive(Debug)]
pub enum GroupType {
    Common,
    Body,
    Action,
    Logic,
    Options,
    Error
}


pub enum GotoVariant {
    Before,
    After,
}

trait Operator {}

pub struct CopyOperator {
    group: GroupType,
    source: String,
}
impl Operator for CopyOperator {}


pub struct GotoOperator {
    dest: GroupType,
    to: GotoVariant
}

impl Operator for GotoOperator {}


pub struct OBTLObject {
    pub name: String,
    pub obj_type: DataType,
    pub groups: Vec<Group>,
    pub operators: Vec<Box<dyn Operator>>
}


pub struct Group {
    group_type: GroupType,
    tokens: Vec<Box<dyn Token>>
}

impl Debug for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Group")
            .field("group_type", &self.group_type)
            .field("tokens", &self.tokens)
            .finish()
    }
}

pub fn parse_obtl_all(file: &str) -> OBTLObject {
    let act = OBTLParser::parse(Rule::act, file).unwrap();
    let mut object = OBTLObject {
        name: "".to_string(),
        obj_type: DataType::Actor,
        groups: vec![],
        operators: vec![],
    };
    for pair in act.into_iter() {
        match pair.as_rule() {
            Rule::object => {
                for tag in pair.into_inner() {
                    match tag.as_rule() {
                        Rule::type_name => {
                            match tag.as_span().as_str() {
                                "ACTOR" => object.obj_type = DataType::Actor,
                                "ITEM" => object.obj_type = DataType::Item,
                                "MATERIAL" => object.obj_type = DataType::Material,
                                &_ => {}
                            }
                        }
                        Rule::token_type => {}
                        Rule::name => {
                            object.name = tag.as_span().as_str().to_string()
                        }
                        Rule::s_value => {}
                        Rule::r_f_value => {}
                        Rule::r_l_value => {}
                        Rule::object => {}
                        Rule::group_type => {}
                        Rule::group => {
                            if tag.clone().into_inner().len() != 0 {
                                let inner = tag.into_inner();
                                let mut group = Group { group_type: GroupType::Common, tokens: vec![] };
                                for pair in inner {
                                    match pair.as_rule() {
                                        Rule::group_type => group.group_type = match pair.as_span().as_str() {
                                            "COMMON" => GroupType::Common,
                                            "BODY" => GroupType::Body,
                                            "ACTION" => GroupType::Action,
                                            "LOGIC" => GroupType::Logic,
                                            "OPTION" => GroupType::Options,
                                            &_ => GroupType::Error
                                        },
                                        Rule::value_token => {
                                            // todo
                                        }
                                        Rule::token => for token in pair.clone().into_inner() {
                                            if pair.clone().into_inner().len() != 0 {
                                                match token.as_rule() {
                                                    Rule::token_type => {
                                                        match group.group_type {
                                                            Common => {
                                                                for token_variant in TCommon::iter() {
                                                                    if token_variant.to_string().as_str() == token.as_span().as_str() {
                                                                        group.tokens.push(Box::new(token_variant))
                                                                    }
                                                                }
                                                            }
                                                            GroupType::Body => {
                                                                for token_variant in TBody::iter() {
                                                                    if token_variant.to_string().as_str() == token.as_span().as_str() {
                                                                        group.tokens.push(Box::new(token_variant))
                                                                    }
                                                                }
                                                            }
                                                            GroupType::Action => {
                                                                for token_variant in TAction::iter() {
                                                                    if token_variant.to_string().as_str() == token.as_span().as_str() {
                                                                        group.tokens.push(Box::new(token_variant))
                                                                    }
                                                                }
                                                            }
                                                            GroupType::Logic => {
                                                                for token_variant in TLogic::iter() {
                                                                    if token_variant.to_string().as_str() == token.as_span().as_str() {
                                                                        group.tokens.push(Box::new(token_variant))
                                                                    }
                                                                }
                                                            }
                                                            GroupType::Options => {
                                                                for token_variant in TOption::iter() {
                                                                    if token_variant.to_string().as_str() == token.as_span().as_str() {
                                                                        group.tokens.push(Box::new(token_variant))
                                                                    }
                                                                }
                                                            }
                                                            GroupType::Error => { /*TODO*/ }
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                object.groups.push(group);
                            }
                        }
                        Rule::token => {}
                        Rule::operator_group_copy => {
                            if tag.clone().into_inner().len() != 0 {
                                let inner = tag.into_inner();
                                let mut operator = CopyOperator { group: GroupType::Common, source: "".to_string() };
                                for pair in inner {
                                    match pair.as_rule() {
                                        Rule::group_type => operator.group = match pair.as_span().as_str() {
                                            "COMMON" => GroupType::Common,
                                            "BODY" => GroupType::Body,
                                            "ACTION" => GroupType::Action,
                                            "LOGIC" => GroupType::Logic,
                                            "OPTION" => GroupType::Options,
                                            &_ => GroupType::Error
                                        },
                                        Rule::name => operator.source = pair.as_span().as_str().to_string(),
                                        _ => {}
                                    }
                                }
                                object.operators.push(Box::new(operator));
                            }
                        }
                        Rule::operator_group_goto => {
                            if tag.clone().into_inner().len() != 0 {
                                let mut operator = GotoOperator { dest: Common, to: Before };
                                let inner = tag.into_inner();
                                for pair in inner {
                                    match pair.as_rule() {
                                        Rule::goto_position => match pair.as_span().as_str() {
                                            "BEFORE" => operator.to = Before,
                                            "AFTER" => operator.to = After,
                                            _ => {}
                                        }
                                        Rule::group_type => match pair.as_span().as_str() {
                                            "COMMON" => operator.dest = GroupType::Common,
                                            "BODY" => operator.dest = GroupType::Body,
                                            "ACTION" => operator.dest = GroupType::Action,
                                            "LOGIC" => operator.dest = GroupType::Logic,
                                            "OPTION" => operator.dest = GroupType::Options,
                                            &_ => operator.dest = GroupType::Error
                                        },
                                        _ => {}
                                    }
                                }
                                object.operators.push(Box::new(operator));
                            }
                            }
                        Rule::act => {}
                        Rule::WHITESPACE => {}
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    object
}


