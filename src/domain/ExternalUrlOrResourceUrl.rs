// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) enum ExternalUrlOrResourceUrl
{
	External
	{
		url: Rc<UrlSerde>,
		title: HashMap<Iso639Dash1Alpha2Language, String>,
	},
	Resource
	{
		url: ResourceUrl,
	}
}

impl ExternalUrlOrResourceUrl
{
	#[inline(always)]
	pub(crate) fn useUrlAndTitle<R, User: FnMut(&Url, &str)-> Result<R, CordialError>>(&self, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, mut user: User) -> Result<R, CordialError>
	{
		use self::ExternalUrlOrResourceUrl::*;
		
		match *self
		{
			External { ref url, ref title } =>
			{
				if let Some(title) = title.get(&iso639Dash1Alpha2Language)
				{
					user(&url.0, title)
				}
				else if let Some(title) = title.get(&fallbackIso639Dash1Alpha2Language)
				{
					user(&url.0, title)
				}
				else
				{
					Err(CordialError::Configuration("No title known for external url even for fallback language".to_owned()))
				}
			},
			
			Resource { ref url } =>
			{
				let (url, title) = ResourceReference
				{
					resource: url.clone(),
					tag: ResourceTag::default,
				}.urlAndAnchorTitleAttribute(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
				user(&url, &title)
			}
		}
	}
}
