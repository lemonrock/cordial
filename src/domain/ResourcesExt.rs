// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait ResourcesExt
{
	#[inline(always)]
	fn urlDataMandatory<'resources>(&'resources self, resourceReference: &ResourceReference, primaryIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<Rc<UrlData>, CordialError>;
}

impl ResourcesExt for Resources
{
	#[inline(always)]
	fn urlDataMandatory<'resources>(&'resources self, resourceReference: &ResourceReference, primaryIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<Rc<UrlData>, CordialError>
	{
		match resourceReference.get(self)
		{
			None => Err(CordialError::Configuration(format!("Missing resource {:?}", resourceReference))),
			Some(resourceRefCell) =>
			{
				let refResource = resourceRefCell.try_borrow()?;
				
				refResource.urlDataMandatory(primaryIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, &resourceReference.tag).map(|urlDataRef| urlDataRef.clone())
			},
		}
	}
}
