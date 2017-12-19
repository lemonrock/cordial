// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum LuaArrayToSassCategorisation
{
	List,
	ColorOrMap,
	NumberOrMap,
	WarningOrMap,
	ErrorOrMap,
	Map,
}

impl LuaArrayToSassCategorisation
{
	#[inline(always)]
	pub(crate) fn convertLuaValueToSassValue(luaValue: &AnyLuaValue) -> Result<SassValue, CordialError>
	{
		match *luaValue
		{
			LuaOther => Err(CordialError::Configuration("LuaOther values are not supported".to_owned())),
			
			LuaArray(ref array) => Self::convertArray(array),
			
			LuaString(ref string) => Ok(SassValue::new_quoted_string(&CString::new(string.clone().into_bytes())?)),
			
			LuaAnyString(ref any_string) => Ok(SassValue::new_quoted_string(&CString::new((&any_string.0[..]).to_vec())?)),
			
			LuaNumber(number) => Ok(SassValue::new_number(number, &CString::new("px").unwrap())),
			
			LuaBoolean(boolean) => Ok(SassValue::new_boolean(boolean)),
			
			LuaNil => Ok(SassValue::new_null()),
		}
	}
	
	#[inline(always)]
	fn convertArray(array: &Vec<(AnyLuaValue, AnyLuaValue)>) -> Result<SassValue, CordialError>
	{
		use self::LuaArrayToSassCategorisation::*;
		
		match Self::categorise(array)
		{
			List => Self::convertList(array),
			
			ColorOrMap => Self::convertColorOrMap(array),
			
			NumberOrMap  => Self::convertNumberOrMap(array),
			
			WarningOrMap => Self::convertWarningOrMap(array),
			
			ErrorOrMap => Self::convertErrorOrMap(array),
			
			Map => Self::convertMap(array),
		}
	}
	
	fn categorise(array: &Vec<(AnyLuaValue, AnyLuaValue)>) -> LuaArrayToSassCategorisation
	{
		use self::LuaArrayToSassCategorisation::*;
		
		let mut nextArrayIndexExpected = Some(1);
		for &(ref key, ref value) in array.iter()
		{
			if let Some(nextArrayIndex) = nextArrayIndexExpected
			{
				if key.isArrayIndex(nextArrayIndex)
				{
					nextArrayIndexExpected = Some(nextArrayIndex + 1);
					continue;
				}
				// NOTE THIS *1
				nextArrayIndexExpected = None;
			}
			
			// NOTE: We do not use "else if" here because of *1 above
			if nextArrayIndexExpected.is_none()
			{
				if key.as_str_is("type")
				{
					if let Some(value) = value.as_str()
					{
						match value
						{
							"color" => return ColorOrMap,
							"number" => return NumberOrMap,
							"warning" => return WarningOrMap,
							"error" => return ErrorOrMap,
							_ => return Map,
						}
					}
					else
					{
						return Map
					}
				}
				else
				{
					return Map
				}
			}
		}
		
		if nextArrayIndexExpected.is_some()
		{
			List
		}
		else
		{
			Map
		}
	}
	
	fn convertList(array: &Vec<(AnyLuaValue, AnyLuaValue)>) -> Result<SassValue, CordialError>
	{
		let sassValue = SassValue::new_list(array.len(), ListSeparator::Comma, false);
		{
			let list = sassValue.as_list().unwrap();
			let mut index = 0;
			for &(ref _key, ref value) in array.iter()
			{
				unsafe { list.set_value_unchecked(index, Self::convertLuaValueToSassValue(value)?) };
				index += 1;
			}
		}
		Ok(sassValue)
	}
	
	fn convertColorOrMap(array: &Vec<(AnyLuaValue, AnyLuaValue)>) -> Result<SassValue, CordialError>
	{
		if array.len() != 5
		{
			return Self::convertMap(array);
		}
		
		let mut red = None;
		let mut green = None;
		let mut blue = None;
		let mut alpha = None;
		for &(ref key, ref value) in array.iter()
		{
			match key.as_str()
			{
				Some("type") => (),
				
				Some("red") => match value.as_f64()
				{
					None => return Self::convertMap(array),
					Some(redValue) =>
					{
						red = Some(redValue)
					}
				}
				
				Some("green") => match value.as_f64()
				{
					None => return Self::convertMap(array),
					Some(greenValue) =>
					{
						green = Some(greenValue)
					}
				}
				
				Some("blue") => match value.as_f64()
				{
					None => return Self::convertMap(array),
					Some(blueValue) =>
					{
						blue = Some(blueValue)
					}
				}
				
				Some("alpha") => match value.as_f64()
				{
					None => return Self::convertMap(array),
					Some(alphaValue) =>
					{
						alpha = Some(alphaValue)
					}
				}
				
				_ => return Self::convertMap(array),
			}
		}
		
		match (red, green, blue, alpha)
		{
			(Some(red), Some(green), Some(blue), Some(alpha)) => Ok(SassValue::new_color(red, green, blue, alpha)),
			_ => unreachable!(),
		}
	}
	
	fn convertNumberOrMap(array: &Vec<(AnyLuaValue, AnyLuaValue)>) -> Result<SassValue, CordialError>
	{
		if array.len() != 3
		{
			return Self::convertMap(array);
		}
		
		let mut numberValue = None;
		let mut numberUnit = None;
		for &(ref key, ref value) in array.iter()
		{
			match key.as_str()
			{
				Some("type") => (),
				
				Some("value") => match value.as_f64()
				{
					None => return Self::convertMap(array),
					Some(valueOfValue) =>
					{
						numberValue = Some(valueOfValue)
					}
				}
				
				Some("unit") => match value.as_str()
				{
					None => return Self::convertMap(array),
					Some(valueOfUnit) =>
					{
						numberUnit = Some(CString::new(valueOfUnit)?)
					}
				}
				
				_ => return Self::convertMap(array),
			}
		}
		
		match (numberValue, numberUnit)
		{
			(Some(numberValue), Some(numberUnit)) => Ok(SassValue::new_number(numberValue, &numberUnit)),
			_ => unreachable!(),
		}
	}
	
	fn convertWarningOrMap(array: &Vec<(AnyLuaValue, AnyLuaValue)>) -> Result<SassValue, CordialError>
	{
		if array.len() != 2
		{
			return Self::convertMap(array);
		}
		
		for &(ref key, ref value) in array.iter()
		{
			match key.as_str()
			{
				Some("type") => (),
				
				Some("message") => match value.as_str()
				{
					None => return Self::convertMap(array),
					
					Some(message) => return Ok(SassValue::new_warning(&CString::new(message)?)),
				},

				_ => return Self::convertMap(array),
			}
		}
		
		unreachable!()
	}
	
	fn convertErrorOrMap(array: &Vec<(AnyLuaValue, AnyLuaValue)>) -> Result<SassValue, CordialError>
	{
		if array.len() != 2
		{
			return Self::convertMap(array);
		}
		
		for &(ref key, ref value) in array.iter()
		{
			match key.as_str()
			{
				Some("type") => (),
				
				Some("message") => match value.as_str()
				{
					None => return Self::convertMap(array),
					
					Some(message) => return Ok(SassValue::new_error(&CString::new(message)?)),
				},
				
				_ => return Self::convertMap(array),
			}
		}
		
		unreachable!()
	}
	
	fn convertMap(array: &Vec<(AnyLuaValue, AnyLuaValue)>) -> Result<SassValue, CordialError>
	{
		let sassValue = SassValue::new_map(array.len());
		{
			let map = sassValue.as_map().unwrap();
			let mut index = 0;
			for &(ref key, ref value) in array.iter()
			{
				unsafe
				{
					map.set_key_unchecked(index, Self::convertLuaValueToSassValue(key)?);
					map.set_value_unchecked(index, Self::convertLuaValueToSassValue(value)?);
				}
				index += 1;
			}
		}
		Ok(sassValue)
	}
}
