use serde_json::{json, Value};

#[inline(always)]
pub fn get_create_schema() -> Value {
    json!(
        {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://cloud.project/create.schema.json",
            "title": "Create",
            "description": "Admin's Create user form",
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
                },
                "access_level": {
                    "description" : "User's access level",
                    "type" : "integer",
                    "minimum" : 0
                }
            },
            "required": [ "email", "password", "first_name", "last_name", "access_level"]
        }
    )
}

#[inline(always)]
pub fn get_update_schema() -> Value {
    json!(
        {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://cloud.project/update.schema.json",
            "title": "Create",
            "description": "Admin's Create user form",
            "type": "object",
            "properties": {
                "email": {
                    "description": "User's email",
                    "type": ["string", "null"],
                    "format": "email"
                },
                "first_name": {
                    "description": "User's name",
                    "type": ["string", "null"]
                },
                "password": {
                    "description": "User's password",
                    "type": ["string", "null"],
                        "minLength": 8,
                        "maxLength": 128
                },
                "first_name": {
                    "description": "User's last name",
                    "type": ["string", "null"]
                }
            },
            "anyOf" : [
                {"required": [ "email" ]},
                {"required": [ "password" ]},
                {"required": [ "first_name" ]},
                {"required": [ "last_name"]},
            ]
        }
    )
}

#[inline(always)]
pub fn get_admin_update_schema() -> Value {
    json!(
        {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://cloud.project/admin.update.schema.json",
            "title": "Create",
            "description": "Admin's Create user form",
            "type": "object",
            "properties": {
                "email": {
                    "description": "User's email",
                    "type": ["string", "null"],
                    "format": "email"
                },
                "first_name": {
                    "description": "User's name",
                    "type": ["string", "null"]
                },
                "password": {
                    "description": "User's password",
                    "type": ["string", "null"],
                        "minLength": 8,
                        "maxLength": 128
                },
                "first_name": {
                    "description": "User's last name",
                    "type": ["string", "null"]
                },
                "access_level": {
                    "description" : "User's access level",
                    "type" : ["string", "null"],
                    "minimum" : 0
                }
            },
            "anyOf" : [
                {"required": [ "email" ]},
                {"required": [ "password" ]},
                {"required": [ "first_name" ]},
                {"required": [ "last_name"]},
                {"required": [ "access_level"]},
            ]
        }
    )
}
