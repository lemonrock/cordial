// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// Combines selector autoprefixers
#[derive(Debug, Clone)]
pub struct CompositeSelectorAutoprefixer
{
}

impl SelectorAutoprefixer for CompositeSelectorAutoprefixer
{
	fn autoprefix(&self, _selectors: &mut DeduplicatedSelectors, _parent_vendor_prefix: Option<&VendorPrefix>)
	{
		unimplemented!();
	}
}

impl CompositeSelectorAutoprefixer
{
	#[inline(always)]
	pub(crate) fn new(_can_i_use: &CanIUse, _our_rules: &AgentNameAndVersionSet) -> Self
	{
		Self
		{
		}
	}
}
