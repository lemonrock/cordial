// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct ColorSassValue<'a>
{
	reference: &'a SassValue,
}

impl<'a> ColorSassValue<'a>
{
	#[inline(always)]
	pub(crate) fn red(&self) -> f64
	{
		unsafe { sass_color_get_r(self.reference.0 as *const _) }
	}
	
	#[inline(always)]
	pub(crate) fn set_red(&self, value: f64)
	{
		unsafe { sass_color_set_r(self.reference.0, value) }
	}
	
	#[inline(always)]
	pub(crate) fn green(&self) -> f64
	{
		unsafe { sass_color_get_g(self.reference.0 as *const _) }
	}
	
	#[inline(always)]
	pub(crate) fn set_green(&self, value: f64)
	{
		unsafe { sass_color_set_g(self.reference.0, value) }
	}
	
	#[inline(always)]
	pub(crate) fn blue(&self) -> f64
	{
		unsafe { sass_color_get_b(self.reference.0 as *const _) }
	}
	
	#[inline(always)]
	pub(crate) fn set_blue(&self, value: f64)
	{
		unsafe { sass_color_set_b(self.reference.0, value) }
	}
	
	#[inline(always)]
	pub(crate) fn alpha(&self) -> f64
	{
		unsafe { sass_color_get_a(self.reference.0 as *const _) }
	}
	
	#[inline(always)]
	pub(crate) fn set_alpha(&self, value: f64)
	{
		unsafe { sass_color_set_a(self.reference.0, value) }
	}
}
