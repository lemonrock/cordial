// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct RobotsTxtConfiguration
{
	relativeUrlPathsForRobotDirective: BTreeSet<Cow<'static, str>>,
	siteMapIndexUrls: BTreeSet<Url>,
}

impl Default for RobotsTxtConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			relativeUrlPathsForRobotDirective: BTreeSet::default(),
			siteMapIndexUrls: BTreeSet::default(),
		}
	}
}

impl RobotsTxtConfiguration
{
	#[inline(always)]
	pub(crate) fn addRelativeUrlPathForRobotDirective(&mut self, languageData: &LanguageData)
	{
		let language = languageData.language;
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		let mut urls = language.robotsTxtRelativeRootUrls(iso639Dash1Alpha2Language);
		
		for url in urls.drain(..)
		{
			self.relativeUrlPathsForRobotDirective.insert(url);
		}
	}
	
	#[inline(always)]
	pub(crate) fn addSiteMapIndexUrl(&mut self, siteMapIndexUrl: &Url)
	{
		self.siteMapIndexUrls.insert(siteMapIndexUrl.clone());
	}
}
