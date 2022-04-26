use serde_json::{json, Value};

#[inline(always)]
pub fn get_sign_in_schema() -> Value {
    json!(
        {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://example.com/product.schema.json",
            "title": "Product",
            "description": "A product from Acme's catalog",
            "type": "object",
            "properties": {
                "email": {
                    "description": "User's email",
                    "type": "string",
                    "format": "email"
                },
                "password": {
                    "description": "User's password",
                    "type": "string",
                        "minLength": 8,
                        "maxLength": 128
                }
            },
            "required": [ "email", "password" ]
        }
    )
}

#[inline(always)]
pub fn get_sign_up_schema() -> Value {
    json!(
        {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://example.com/product.schema.json",
            "title": "Product",
            "description": "A product from Acme's catalog",
            "type": "object",
            "properties": {
                "email": {
                    "description": "User's email",
                    "type": "string",
                    "format": "email"
                },
                "password": {
                    "description": "User's password",
                    "type": "string",
                        "minLength": 8,
                        "maxLength": 128
                },
                "first_name": {
                    "description": "User's name",
                    "type": "string"
                },
                "first_name": {
                    "description": "User's last name",
                    "type": "string"
                }
            },
            "required": [ "email", "password", "first_name", "last_name"]
        }
    )
}
