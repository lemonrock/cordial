// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum RequiredTranslation
{
	missing_image_fallback,
	missing_video_fallback,
	your_browser_does_not_support_video,
}

impl RequiredTranslation
{
	#[inline(always)]
	pub(crate) fn englishTranslations() -> HashMap<RequiredTranslation, Rc<String>>
	{
		use self::RequiredTranslation::*;
		
		fn text(text: &'static str) -> Rc<String>
		{
			Rc::new(text.to_owned())
		}
		
		hashmap!
		{
			missing_image_fallback => text("Unfortunately, this image is unavailable at this time."),
			missing_video_fallback => text("Unfortunately, this video is unavailable at this time."),
			your_browser_does_not_support_video => text("Unfortunately, your browser does not support video in the formats we use."),
		}
	}
}
