// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Serialize, Debug, Copy, Clone)]
pub(crate) struct LanguageData<'a>
{
	pub(crate) iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language,
	pub(crate) language: &'a Language,
}

impl<'a> LanguageData<'a>
{
	pub(crate) fn new(iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, language: &'a Language) -> Self
	{
		Self
		{
			iso639Dash1Alpha2Language,
			language,
		}
	}

	#[inline(always)]
	pub(crate) fn baseUrl(&self, isForAmp: bool) -> Result<Url, CordialError>
	{
		self.language.baseUrl(self.iso639Dash1Alpha2Language, isForAmp)
	}
}
