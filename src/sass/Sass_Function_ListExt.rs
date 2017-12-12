// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait Sass_Function_ListExt
{
	#[inline(always)]
	fn make(size: usize) -> Self;
	
	#[inline(always)]
	fn set_list_entry(self, index: usize, function_entry: Sass_Function_Entry);
	
	#[inline(always)]
	fn delete(self);
}

impl Sass_Function_ListExt for Sass_Function_List
{
	#[inline(always)]
	fn make(size: usize) -> Self
	{
		unsafe { sass_make_function_list(size) }
	}
	
	#[inline(always)]
	fn set_list_entry(self, index: usize, function_entry: Sass_Function_Entry)
	{
		unsafe { sass_function_set_list_entry(self, index, function_entry) }
	}
	
	#[inline(always)]
	fn delete(self)
	{
		unsafe { sass_delete_function_list(self) }
	}
}
