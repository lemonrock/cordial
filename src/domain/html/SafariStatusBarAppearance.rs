// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// Sourced from https://developers.google.com/web/fundamentals/design-and-ux/browser-customization/#change_the_status_bar_appearance
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SafariStatusBarAppearance
{
	default,
	black,
	black_translucent
}

impl Default for SafariStatusBarAppearance
{
	#[inline(always)]
	fn default() -> Self
	{
		SafariStatusBarAppearance::default
	}
}

impl SafariStatusBarAppearance
{
	#[inline(always)]
	pub(crate) fn addTo(&self, startHeadNodes: &mut Vec<UnattachedNode>)
	{
		use self::SafariStatusBarAppearance::*;
		
		match *self
		{
			black => startHeadNodes.push(meta_with_name_and_content("apple-mobile-web-app-status-bar-style", "black")),
			black_translucent => startHeadNodes.push(meta_with_name_and_content("apple-mobile-web-app-status-bar-style", "black-translucent")),
			_ => (),
		}
	}
}
