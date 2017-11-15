// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait JsonValueExt
{
	#[inline(always)]
	fn u32(&self, name: &str) -> Result<u32, CordialError>;
	
	#[inline(always)]
	fn u64(&self, name: &str) -> Result<u64, CordialError>;
	
	#[inline(always)]
	fn mime(&self, name: &str) -> Result<Mime, CordialError>;
}

impl JsonValueExt for Rc<JsonValue>
{
	#[inline(always)]
	fn u32(&self, name: &str) -> Result<u32, CordialError>
	{
		match self.get(name)
		{
			None => Err(CordialError::Configuration(format!("Missing name '{}'", name))),
			Some(value) => match value.as_u64()
			{
				None => Err(CordialError::Configuration(format!("Is not u64 value '{:?}'", value))),
				Some(value) => if value > (2^32 - 1)
				{
					Err(CordialError::Configuration(format!("Is not u32 value in range 0 - 2^32 - 1 '{:?}'", value)))
				}
				else
				{
					Ok(value as u32)
				}
			}
		}
	}
	
	#[inline(always)]
	fn u64(&self, name: &str) -> Result<u64, CordialError>
	{
		match self.get(name)
		{
			None => Err(CordialError::Configuration(format!("Missing name '{}'", name))),
			Some(value) => match value.as_u64()
			{
				None => Err(CordialError::Configuration(format!("Is not u64 value '{:?}'", value))),
				Some(value) => Ok(value)
			}
		}
	}
	
	#[inline(always)]
	fn mime(&self, name: &str) -> Result<Mime, CordialError>
	{
		match self.get(name)
		{
			None => Err(CordialError::Configuration(format!("Missing name '{}'", name))),
			Some(value) => match value.as_str()
			{
				None => Err(CordialError::Configuration(format!("Is not string value '{:?}'", value))),
				Some(value) =>
				{
					match value.parse()
					{
						Err(_) => Err(CordialError::Configuration(format!("MIME invalid '{}'", name))),
						Ok(mime) => Ok(mime),
					}
				}
			}
		}
	}
}
