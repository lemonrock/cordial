// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ResourceReference
{
	#[serde(default)] pub(crate) resource: ResourceUrl,
	#[serde(default)] pub(crate) tag: ResourceTag,
}

impl Default for ResourceReference
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			resource: Default::default(),
			tag: Default::default(),
		}
	}
}

impl ResourceReference
{
	#[inline(always)]
	pub(crate) fn new<S: Into<String>>(resource: S, tag: ResourceTag) -> Self
	{
		Self
		{
			resource: ResourceUrl::string(resource),
			tag,
		}
	}
	
	#[inline(always)]
	pub(crate) fn get<'resources>(&self, resources: &'resources Resources) ->  Option<&'resources RefCell<Resource>>
	{
		self.resource.get(resources)
	}
	
	#[inline(always)]
	pub(crate) fn getUrlData<'resources>(&self, resources: &'resources Resources, primaryIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) ->  Result<Option<(Rc<UrlData>, Ref<'resources, Resource>)>, CordialError>
	{
		match self.get(resources)
		{
			None => Ok(None),
			Some(resource) =>
			{
				let borrowedResource = resource.try_borrow()?;
				let urlData = borrowedResource.urlDataMandatory(primaryIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, &self.tag)?.clone();
				Ok(Some((urlData, borrowedResource)))
			}
		}
	}
}
