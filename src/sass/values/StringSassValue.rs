// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct StringSassValue<'a>
{
	reference: &'a SassValue,
}

impl<'a> StringSassValue<'a>
{
	#[inline(always)]
	pub(crate) fn value(&self) -> *const c_char
	{
		unsafe { sass_string_get_value(self.reference.0 as *const _) }
	}
	
	#[inline(always)]
	pub(crate) fn set_value(&self, value: *mut c_char)
	{
		unsafe { sass_string_set_value(self.reference.0, value) }
	}
	
	#[inline(always)]
	pub(crate) fn is_quoted(&self) -> bool
	{
		unsafe { sass_string_is_quoted(self.reference.0 as *const _) }
	}
	
	#[inline(always)]
	pub(crate) fn set_is_quoted(&self, is_quoted: bool)
	{
		unsafe { sass_string_set_quoted(self.reference.0, is_quoted) }
	}
}
