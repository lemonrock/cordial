// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait Sass_Data_ContextExt
{
	#[inline(always)]
	fn make(data: &CStr) -> Self;
	
	#[inline(always)]
	fn delete(self);
	
	#[inline(always)]
	fn compile(self) -> i32;
	
	#[inline(always)]
	fn get_context(self) -> *mut Sass_Context;
	
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options;
}

impl Sass_Data_ContextExt for *mut Sass_Data_Context
{
	#[inline(always)]
	fn make(data: &CStr) -> Self
	{
		unsafe { sass_make_data_context(strdup(data.as_ptr())) }
	}
	
	#[inline(always)]
	fn delete(self)
	{
		unsafe { sass_delete_data_context(self) };
	}
	
	#[inline(always)]
	fn compile(self) -> i32
	{
		unsafe { sass_compile_data_context(self) }
	}
	
	#[inline(always)]
	fn get_context(self) -> *mut Sass_Context
	{
		unsafe { sass_data_context_get_context(self) }
	}
	
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options
	{
		unsafe { sass_data_context_get_options(self) }
	}
}
