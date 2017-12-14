// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct ErrorSassValue<'a>
{
	reference: &'a SassValue,
}

impl<'a> ErrorSassValue<'a>
{
	#[inline(always)]
	pub(crate) fn message(&self) -> *const c_char
	{
		unsafe { sass_error_get_message(self.reference.0 as *const _) }
	}
	
	#[inline(always)]
	pub(crate) fn set_message(&self, message: *mut c_char)
	{
		unsafe { sass_error_set_message(self.reference.0, message) }
	}
}
