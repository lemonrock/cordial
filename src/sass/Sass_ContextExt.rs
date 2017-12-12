// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait Sass_ContextExt
{
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options;
	
	#[inline(always)]
	fn get_error_status(self) -> i32;
	
	#[inline(always)]
	fn get_error_message(self) -> *const c_char;
	
	#[inline(always)]
	fn get_output_string(self) -> *const c_char;
}

impl Sass_ContextExt for *mut Sass_Context
{
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options
	{
		unsafe { sass_context_get_options(self) }
	}
	
	#[inline(always)]
	fn get_error_status(self) -> i32
	{
		unsafe { sass_context_get_error_status(self) }
	}
	
	#[inline(always)]
	fn get_error_message(self) -> *const c_char
	{
		unsafe { sass_context_get_error_message(self) }
	}
	
	#[inline(always)]
	fn get_output_string(self) -> *const c_char
	{
		unsafe { sass_context_get_output_string(self) }
	}
}
