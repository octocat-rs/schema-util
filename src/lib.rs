use crate::model::Type;

pub mod model;
pub mod traits;

pub(crate) fn types_to_string(types: &[Type; 2]) -> String {
    // We shouldn't need to initialize this, but Clippy gets confused otherwise.
    let mut ret = "".to_owned();

    if types.contains(&Type::Null) {
        for ty in types {
            if ty != &Type::Null {
                ret = format!("Option<{}>", ty);
                // This is unnecessary, but Clippy gets confused without it.
                break;
            }
        }
    } else {
        ret = format!("Either<{}, {}>", types[0], types[1]);
    }

    ret
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{Model, Modeler, Type},
        traits::PrintAsStruct,
    };
    use std::{collections::HashMap, error::Error, fs};

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        let issue_text = fs::read_to_string("test.json")?;
        let issue_json = serde_json::from_str::<Modeler>(issue_text.as_str())?;

        println!("{}", issue_json.print());

        Ok(())
    }
}
