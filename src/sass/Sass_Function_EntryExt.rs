// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait Sass_Function_EntryExt
{
	#[inline(always)]
	fn get_cookie(self) -> *mut c_void;
}

impl Sass_Function_EntryExt for Sass_Function_Entry
{
	#[inline(always)]
	fn get_cookie(self) -> *mut c_void
	{
		unsafe { sass_function_get_cookie(self) as *mut c_void }
	}
}
