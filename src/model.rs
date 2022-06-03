use crate::{traits::PrintAsStruct, types_to_string};
use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize)]
pub struct Modeler {
    pub title: String,
    pub properties: HashMap<String, Model>,
    pub required: Vec<String>,
}

impl PrintAsStruct for Modeler {
    fn print(self) -> String {
        let mut ret = "#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]".to_owned();
        ret.push_str(format!("\npub struct {} {{", self.title.to_upper_camel_case()).as_str());

        let title_to_type = |s: &str| match s {
            "Simple User" | "Public User" | "Private User" | "Reaction Rollup" => s.to_upper_camel_case(),
            "Repository" | "Milestone" => s.to_owned(),
            "License Simple" => "SimpleLicense".to_owned(),
            "author_association" => "Association".to_owned(),
            _ => "Object".to_owned(),
        };

        self.required.into_iter().for_each(|field| {
            let model = self.properties.get(&field).unwrap();

            let ty: String = match model {
                Model::Type { type_field } => type_field.to_string(),
                Model::MultiType { types } => types_to_string(types),
                Model::TitledObject { title, .. } => title_to_type(title),
                Model::AnyOf { any_of: types } => {
                    let to_type = |ty: &Model| match ty {
                        Model::Type { type_field } => *type_field,
                        _ => unimplemented!(),
                    };

                    let types = types.iter().map(to_type).collect::<Vec<Type>>();

                    types_to_string(&[types[0], types[1]])
                }
            };

            if field.as_str() != "type" {
                ret.push_str(format!("\n    pub {}: {},", field, ty).as_str());
            } else {
                ret.push_str("\n    #[serde(rename = \"type\")]");
                ret.push_str(format!("\n    pub type_field: {},", ty).as_str());
            }
        });

        ret.push_str("\n}");

        ret
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Model {
    Type {
        #[serde(rename = "type")]
        type_field: Type,
    },
    MultiType {
        #[serde(rename = "type")]
        types: [Type; 2],
    },
    TitledObject {
        title: String,
        #[serde(rename = "type")]
        type_field: Type,
    },
    AnyOf {
        /// The vec should only contain values of variant [`Model::Type`] and/or
        /// [`Model::TitledObject`]
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
