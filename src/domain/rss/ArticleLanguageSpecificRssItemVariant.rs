// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct ArticleLanguageSpecificRssItemVariant
{
	rssTitle: Rc<String>,
	rssDescription: Vec<u8>,
}

impl ArticleLanguageSpecificRssItemVariant
{
	#[inline(always)]
	pub(crate) fn titleDescriptionContentEncodedAndPublicationDate<R, User: FnMut(&str, &str, Option<&str>, Option<DateTime<Utc>>) -> Result<R, CordialError>>(&self, mut user: User, lastModifiedDate: Option<DateTime<Utc>>) -> Result<R, CordialError>
	{
		user(&self.rssTitle, unsafe { from_utf8_unchecked(&self.rssDescription) }, None, lastModifiedDate)
	}
}
