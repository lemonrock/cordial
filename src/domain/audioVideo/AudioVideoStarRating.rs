// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AudioVideoStarRating(u8);

impl<'de> Deserialize<'de> for AudioVideoStarRating
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		let value = u8::deserialize(deserializer)?;
		if value > 50
		{
			return Err(D::Error::custom("value exceeds 50"))
		}
		Ok(AudioVideoStarRating(value))
	}
}

impl AudioVideoStarRating
{
	#[inline(always)]
	pub(crate) fn toGoogleSiteMapString(&self) -> String
	{
		let major = self.0 / 10;
		let minor = self.0 % 10;
		format!("{}.{}", major, minor)
	}
}
