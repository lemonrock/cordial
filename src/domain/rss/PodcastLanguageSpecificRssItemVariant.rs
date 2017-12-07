// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct PodcastLanguageSpecificRssItemVariant
{
	description: Rc<String>,
	episode_note_html: Vec<u8>, // Is is limited to 255 characters?
}

impl PodcastLanguageSpecificRssItemVariant
{
	#[inline(always)]
	pub(crate) fn descriptionAndContentEncoded(&self) -> (&str, Option<&str>)
	{
		(&self.description, Some(unsafe { from_utf8_unchecked(&self.episode_note_html) }))
	}
}
