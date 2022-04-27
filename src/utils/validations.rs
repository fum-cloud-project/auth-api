use jsonschema::JSONSchema;
use serde_json::Value;

pub fn validate(schema: Value, from_api: Value) -> Result<(), Vec<String>> {
    let compiled = JSONSchema::compile(&schema).expect("A valid schema");
    let result = compiled.validate(&from_api);
    let mut res: Vec<String> = vec![];
    match result {
        Ok(_) => {
            return Ok(());
        }
        Err(errors) => {
            for error in errors {
                res.push(format!("{}", error));
            }
            return Err(res);
        }
    }
}
