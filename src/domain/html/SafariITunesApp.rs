// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct SafariITunesApp
{
	id: String, // eg 828256236
	affiliate: Option<String>,
	argument: Option<String>, // if not supplied, the current URL
}

impl SafariITunesApp
{
	#[inline(always)]
	pub(crate) fn addToEndHeadNodes(&self, endHeadNodes: &mut Vec<UnattachedNode>, resourceUrl: &ResourceUrl)
	{
		let appArgument = if let Some(ref argument) = self.argument
		{
			argument
		}
		else
		{
			resourceUrl.deref()
		};
		
		let content = if let Some(ref affiliate) = self.affiliate
		{
			format!("app-id={}, affiliate-data={}, app-argument={}", &self.id, affiliate, appArgument)
		}
		else
		{
			format!("app-id={}, app-argument={}", &self.id, appArgument)
		};
		
		endHeadNodes.push(meta_with_name_and_content("apple-itunes-app", &content));
	}
}
