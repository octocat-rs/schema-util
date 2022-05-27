use crate::{traits::PrintAsStruct, types_to_string};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize)]
pub struct Modeler {
    pub title: String,
    pub properties: HashMap<String, Model>,
}

impl PrintAsStruct for Modeler {
    fn print(self) -> String {
        let mut ret = "#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]".to_owned();
        ret.push_str(format!("\npub struct {} {{", self.title).as_str());

        self.properties.iter().for_each(|(key, val)| {
            let ty: String = match val {
                Model::Type { type_field } => type_field.to_string(),
                Model::MultiType { types } => types_to_string(types),
                Model::AnyOf { any_of: types } => {
                    let to_type = |ty: &Model| match ty {
                        Model::Type { type_field } => *type_field,
                        _ => unimplemented!(),
                    };

                    let types = types.iter().map(to_type).collect::<Vec<Type>>();

                    types_to_string(&[types[0], types[1]])
                }
            };

            ret.push_str(format!("\n\t{}: {},", key, ty).as_str());
        });

        ret
    }
}

/// Use HashMap
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Model {
    Type {
        #[serde(rename = "type")]
        type_field: Type,
    },
    MultiType {
        /// The vec should only contain values of variant [`Model::Type`]
        #[serde(rename = "type")]
        types: [Type; 2],
    },
    AnyOf {
        /// The vec should only contain values of variant [`Model::Type`]
        #[serde(rename = "anyOf")]
        any_of: Box<[Model; 2]>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    String,
    #[strum(serialize = "i64")]
    Integer,
    #[strum(serialize = "bool")]
    Boolean,
    Object,
    Array,
    Null,
}
