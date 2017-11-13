// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// Accurate as of Media Queries 4 (https://drafts.csswg.org/mediaqueries/#media-types)
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone)]
pub enum MediaType
{
	all,
	print,
	screen,
	speech,
}

impl Default for MediaType
{
	#[inline(always)]
	fn default() -> Self
	{
		MediaType::screen
	}
}
