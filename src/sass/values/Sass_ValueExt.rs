// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait Sass_ValueExt
{
	#[inline(always)]
	fn delete(self);
	
	#[inline(always)]
	fn stringify(self, compressed: bool, precision: u8) -> *mut Sass_Value;
	
	#[inline(always)]
	fn tag(self) -> Sass_Tag;
}

impl Sass_ValueExt for *mut Sass_Value
{
	#[inline(always)]
	fn delete(self)
	{
		unsafe { sass_delete_value(self) }
	}
	
	#[inline(always)]
	fn stringify(self, compressed: bool, precision: u8) -> *mut Sass_Value
	{
		unsafe { sass_value_stringify(self, compressed, precision as i32) }
	}
	
	#[inline(always)]
	fn tag(self) -> Sass_Tag
	{
		unsafe { sass_value_get_tag(self) }
	}
}
