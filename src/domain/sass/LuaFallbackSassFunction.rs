// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct LuaFallbackSassFunction
{
	luaShortCodeHelper: LuaShortCodeHelper,
}

impl SassFunction for LuaFallbackSassFunction
{
	#[inline(always)]
	fn signature(&self) -> SassFunctionSignature
	{
		SassFunctionSignature::Fallback
	}
	
	fn callback(&mut self, arguments: ListSassValue, _compiler: SassCompiler) -> Result<SassValue, SassValueError>
	{
		if arguments.length() < 2
		{
			return SassValueError::function_failed_from_static_str("Must have at least one argument, the name of the function we are falling back for");
		}
		
		let mut arguments = arguments.into_iter();
		let shortCodeName = arguments.next().unwrap();
		let shortCodeName = shortCodeName.as_string()?.value();
		let shortCodeName = shortCodeName.to_str()?;
		
		if shortCodeName.is_empty()
		{
			return SassValueError::function_failed_from_static_str("fallback function name was empty");
		}
		
		match self.luaShortCodeHelper.callFromSass(shortCodeName, &mut arguments)
		{
			Err(error) => SassValueError::function_failed_from_string(format!("Lua failed with '{}'", error)),
			Ok(data) => Ok(data),
		}
	}
}
