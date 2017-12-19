// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait AnyLuaValueExt
{
	#[inline(always)]
	fn isArrayIndex(&self, matches: u64) -> bool;
	
	#[inline(always)]
	fn as_f64(&self) -> Option<f64>;
	
	#[inline(always)]
	fn as_str(&self) -> Option<&str>;
	
	#[inline(always)]
	fn as_str_is(&self, matches: &str) -> bool;
}

impl AnyLuaValueExt for AnyLuaValue
{
	#[inline(always)]
	fn isArrayIndex(&self, matches: u64) -> bool
	{
		match *self
		{
			LuaNumber(number) => number.is_sign_positive() && number.is_finite() && number > 0.0f64 && number.trunc() == number && (number as u64 == matches),
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn as_f64(&self) -> Option<f64>
	{
		match *self
		{
			LuaNumber(value) => Some(value),
			_ => None,
		}
	}
	
	#[inline(always)]
	fn as_str(&self) -> Option<&str>
	{
		match *self
		{
			LuaString(ref str) => Some(str),
			_ => None,
		}
	}
	
	#[inline(always)]
	fn as_str_is(&self, matches: &str) -> bool
	{
		if let Some(value) = self.as_str()
		{
			value == matches
		}
		else
		{
			false
		}
	}
}
