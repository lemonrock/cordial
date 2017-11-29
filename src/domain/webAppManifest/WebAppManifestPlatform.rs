// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// Source: https://github.com/w3c/manifest/wiki/Platforms
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum WebAppManifestPlatform
{
	play,
	itunes,
	windows,
}

impl Default for WebAppManifestPlatform
{
	#[inline(always)]
	fn default() -> Self
	{
		WebAppManifestPlatform::play
	}
}
