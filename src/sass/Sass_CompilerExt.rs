// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait Sass_CompilerExt
{
	#[inline(always)]
	fn get_context(self) -> *mut Sass_Context;
	
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options;
	
	#[inline(always)]
	fn get_last_import(self) -> Sass_Import_Entry;
}

impl Sass_CompilerExt for *mut Sass_Compiler
{
	#[inline(always)]
	fn get_context(self) -> *mut Sass_Context
	{
		unsafe { sass_compiler_get_context(self) }
	}
	
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options
	{
		unsafe { sass_compiler_get_options(self) }
	}
	
	#[inline(always)]
	fn get_last_import(self) -> Sass_Import_Entry
	{
		unsafe { sass_compiler_get_last_import(self) }
	}
}
