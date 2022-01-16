use std::{convert::TryFrom, collections::HashMap};
use anyhow::Result;
use wither::bson;
use serde_json::{Map, Value};

fn conditional_key(key: &str) -> bool {
    match key {
        // Comparison
        "_eq" => true,
        "_gt" => true,
        "_gte" => true,
        "_in" => true,
        "_lt" => true,
        "_lte" => true,
        "_ne" => true,
        "_nin" => true,

        // Logical
        "_and" => true,
        "_not" => true,
        "_nor" => true,
        "_or" => true,

        // Element
        "_exists" => true,
        "_type" => true,

        // Evaluation
        "_expr" => true,
        "_jsonSchema" => true,
        "_mod" => true,
        "_regex" => true,
        "_text" => true,
        "_where" => true,

        // GeoSpatial
        "_geoIntersects" => true,
        "_geoWithin" => true,
        "_near" => true,
        "_nearSphere" => true,
        "_box" => true,
        "_center" => true,
        "_centerSphere" => true,
        "_geometry" => true,
        "_maxDistance" => true,
        "_minDistance" => true,
        "_polygon" => true,
        "_uniqueDocs" => true,

        // Array
        "_all" => true,
        "_elemMatch" => true,
        "_size" => true,

        // Bitwise
        "_bitsAllClear" => true,
        "_bitsAllSet" => true,
        "_bitsAnyClear" => true,
        "_bitsAnySet" => true,

        // Projection
        "_projection" => true,
        //"_elemMatch" => true,   //Duplicate
        "_slice" => true,

        // Miscellaneous
        "_comment" => true,
        "_rand" => true,

        _ => false
    }
}

pub fn format_filter(data: &Map<String, Value>) -> Map<String, Value> {
    let mut map = Map::<String, Value>::new();

    for (key, value) in data.iter() {
        let mut new_key = key.clone();
        let mut new_value = value.clone();

        if conditional_key(key) {
            new_key = str::replace(key, "_", "$");
        }

        if value.is_object() {
            let value = format_filter(&value.as_object().unwrap());
            new_value = Value::Object(value.clone());
        }

        if value.is_array() {
            let mut new_array = Vec::<Value>::new();

            for item in value.as_array().unwrap().iter() {
                if item.is_object() {
                    let v = format_filter(&item.as_object().unwrap());
                    new_array.push(Value::Object(v.clone()));
                }
            }

            new_value = Value::Array(new_array);
        }

        map.insert(new_key, new_value.clone());
    }

    map
}

pub fn process_filter(filter: HashMap<String, Value>) -> Result<bson::Document> {
    let filter: Map<String, Value> = filter.into_iter().collect();
    let filter = format_filter(&filter);
    let filter = bson::Document::try_from(filter)?;

    Ok(filter)
}