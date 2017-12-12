// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait Sass_Import_EntryExt
{
	#[inline(always)]
	fn get_abs_path(self) -> *const c_char;
	
	#[inline(always)]
	fn get_imp_path(self) -> *const c_char;
}

impl Sass_Import_EntryExt for Sass_Import_Entry
{
	#[inline(always)]
	fn get_abs_path(self) -> *const c_char
	{
		unsafe { sass_import_get_abs_path(self) }
	}
	
	#[inline(always)]
	fn get_imp_path(self) -> *const c_char
	{
		unsafe { sass_import_get_imp_path(self) }
	}
}
