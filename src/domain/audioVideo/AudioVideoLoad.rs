// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum AudioVideoLoad
{
	auto_play,
	auto,
	metadata,
	none,
	browser_default,
}

impl Default for AudioVideoLoad
{
	#[inline(always)]
	fn default() -> Self
	{
		AudioVideoLoad::browser_default
	}
}

impl AudioVideoLoad
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn addToAudioOrVideoNode(&self, audioOrVideoNode: UnattachedNode, durationInSeconds: u64) -> UnattachedNode
	{
		use self::AudioVideoLoad::*;
		
		match *self
		{
			// Twitter Player Card: Content greater than 10 seconds in length must not automatically play; we 'downgrade' to preload=auto
			auto_play => if durationInSeconds > 10
			{
				audioOrVideoNode.with_empty_attribute("preload")
			}
			else
			{
				audioOrVideoNode.with_empty_attribute("autoplay")
			},
			auto => audioOrVideoNode.with_empty_attribute("preload"),
			metadata => audioOrVideoNode.with_attribute("preload".str_attribute("metadata")),
			none => audioOrVideoNode.with_attribute("preload".str_attribute("none")),
			browser_default => audioOrVideoNode,
		}
	}
}
