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
	pub(crate) fn resourceMandatory<'resources>(&self, resources: &'resources Resources) ->  Result<Ref<'resources, Resource>, CordialError>
	{
		self.resource.resourceMandatory(resources)
	}
	
	#[inline(always)]
	pub(crate) fn urlDataMandatory<'resources>(&self, resources: &'resources Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) ->  Result<Rc<UrlData>, CordialError>
	{
		let borrowedResource = self.resourceMandatory(resources)?;
		borrowedResource.urlDataMandatory(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, &self.tag).map(|urlData| urlData.clone())
	}
	
	#[inline(always)]
	pub(crate) fn urlDataAndResourceMandatory<'resources>(&self, resources: &'resources Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) ->  Result<(Rc<UrlData>, Ref<'resources, Resource>), CordialError>
	{
		let borrowedResource = self.resourceMandatory(resources)?;
		let urlData = borrowedResource.urlDataMandatory(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, &self.tag)?.clone();
		Ok((urlData, borrowedResource))
	}
	
	#[inline(always)]
	pub(crate) fn urlMandatory<'resources>(&self, resources: &'resources Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) ->  Result<(Rc<Url>), CordialError>
	{
		let borrowedResource = self.resourceMandatory(resources)?;
		borrowedResource.urlDataMandatory(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, &self.tag).map(|urlData| urlData.url().clone())
	}
	
	#[inline(always)]
	pub(crate) fn urlAndAnchorTitleAttribute<'resources>(&self, resources: &'resources Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(Rc<Url>, Rc<String>), CordialError>
	{
		let (urlData, resource) = self.urlDataAndResourceMandatory(resources, fallbackIso639Dash1Alpha2Language,Some(iso639Dash1Alpha2Language))?;
		
		match resource.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?
		{
			None => Err(CordialError::Configuration(format!("This resource '{:?}' does not have an anchor title attribute", self))),
			Some(anchorTitleAttribute) => Ok((urlData.url().clone(), anchorTitleAttribute))
		}
	}
}
