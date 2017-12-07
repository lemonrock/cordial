// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AudioVolume(u8);

impl Default for AudioVolume
{
	#[inline(always)]
	fn default() -> Self
	{
		AudioVolume(Self::MaximumInclusive)
	}
}

impl<'de> Deserialize<'de> for AudioVolume
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		let value = u8::deserialize(deserializer)?;
		if value > Self::MaximumInclusive
		{
			return Err(D::Error::custom("value is over 10"))
		}
		Ok(AudioVolume(value))
	}
}

impl AudioVolume
{
	const MaximumInclusive: u8 = 10;
	
	#[inline(always)]
	pub(crate) fn writeToAudioNode(&self, audioNode: UnattachedNode) -> UnattachedNode
	{
		if self.0 != Self::MaximumInclusive
		{
			audioNode.with_attribute("volume".string_attribute(format!("{}.{}", self.0 / 10, self.0 % 10)))
		}
		else
		{
			audioNode
		}
	}
}
