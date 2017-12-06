// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum ResourceTag
{
	default,
	
	smallest_image,
	largest_image,
	primary_image,
	width_image(u32),
	height_image(u32),
	width_height_image(u32, u32),
	
	// Value is index in source set
	animation_placeholder(usize),
	
	redirect,
	amp,
	amp_redirect,

	audio_preview,
	
	audio_mp3,

	video_mp4,
	video_webm,
	video_iframe,
	video_track(AudioVideoTrackKind, Iso639Dash1Alpha2Language),
}

impl Default for ResourceTag
{
	#[inline(always)]
	fn default() -> Self
	{
		ResourceTag::default
	}
}
