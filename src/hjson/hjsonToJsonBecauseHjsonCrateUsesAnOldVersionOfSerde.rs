// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use ::serde_json::Map as JsonMap;
use ::serde_json::Number as JsonNumber;
use ::serde_json::Value as JsonValue;


pub fn hjsonToJsonBecauseHjsonCrateUsesAnOldVersionOfSerde(mut hjsonValue: HjsonValue) -> JsonValue
{
	use self::HjsonValue::*;
	match hjsonValue
	{
		Null => JsonValue::Null,
		Bool(value) => JsonValue::Bool(value),
		I64(value) => JsonValue::Number(JsonNumber::from(value)),
		U64(value) => JsonValue::Number(JsonNumber::from(value)),
		F64(value) => JsonValue::Number(JsonNumber::from_f64(value).expect("HJSON parser should be applying the same rules to floating point numbers as JSON parser")),
		String(value) => JsonValue::String(value),
		Array(mut vectorOfValues) =>
		{
			let mut jsonVectorOfValues = Vec::with_capacity(vectorOfValues.len());
			for value in vectorOfValues.drain(..)
			{
				jsonVectorOfValues.push(hjsonToJsonBecauseHjsonCrateUsesAnOldVersionOfSerde(value))
			}
			JsonValue::Array(jsonVectorOfValues)
		}
		Object(mut mapOfStringKeysToValues) =>
		{
			let length = mapOfStringKeysToValues.len();
			let mut jsonMapOfStringKeysToValues = JsonMap::with_capacity(length);
			
			// NOTE: This horrid-looking, inefficient design is because the version of linked_hash_map exported by the hjson crate is too old, lacks entries() and does not have drain()
			let mut keys = Vec::with_capacity(length);
			for key in mapOfStringKeysToValues.keys()
			{
				keys.push(key.to_owned());
			}
			for key in keys.drain(..)
			{
				let hjsonValue = mapOfStringKeysToValues.remove(&key).unwrap();
				jsonMapOfStringKeysToValues.insert(key, hjsonToJsonBecauseHjsonCrateUsesAnOldVersionOfSerde(hjsonValue));
			}
			
			JsonValue::Object(jsonMapOfStringKeysToValues)
		}
	}
}
