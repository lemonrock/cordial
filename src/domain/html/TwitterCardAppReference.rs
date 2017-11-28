// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct TwitterCardAppReference
{
	name: String,
	id: String,
	url: Option<String>, // Not necessarily a real URL
}

impl TwitterCardAppReference
{
	#[inline(always)]
	fn addToIf(this: &Option<Self>, storeName: &str, endHeadNodes: &mut Vec<UnattachedNode>) -> bool
	{
		if let &Some(ref this) = this
		{
			this.addTo(endHeadNodes, storeName);
			false
		}
		else
		{
			true
		}
	}
	
	#[inline(always)]
	fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>, appStoreName: &str)
	{
		endHeadNodes.push(meta_with_name_and_content(&format!("twitter:app:name:{}", appStoreName), &self.name));
		endHeadNodes.push(meta_with_name_and_content(&format!("twitter:app:id:{}", appStoreName), &self.id));
		if let Some(ref url) = self.url
		{
			endHeadNodes.push(meta_with_name_and_content(&format!("twitter:app:url:{}", appStoreName), url.as_str()));
		}
	}
}
