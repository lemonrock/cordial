// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum AmpLayout
{
	nodisplay,
	fixed,
	responsive,
	fixed_height,
	fill,
	container,
	flex_item,
}

impl Deref for AmpLayout
{
	type Target = str;
	
	//noinspection SpellCheckingInspection
	fn deref(&self) -> &Self::Target
	{
		use self::AmpLayout::*;
		
		match *self
		{
			nodisplay => "nodisplay",
			fixed => "fixed",
			responsive => "responsive",
			fixed_height => "fixed-height",
			fill => "fill",
			container => "container",
			flex_item => "flex-item",
		}
	}
}

impl AmpLayout
{
	#[inline(always)]
	pub(crate) fn toAttribute(&self) -> Attribute
	{
		"layout".str_attribute(self.deref())
	}
}
