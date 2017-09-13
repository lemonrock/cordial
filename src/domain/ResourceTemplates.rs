// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) struct ResourceTemplates
{
	resourceTemplate: HjsonValue,
	overridesOfResourceTemplate: HashMap<Vec<String>, HjsonValue>
}

impl ResourceTemplates
{
	#[inline(always)]
	pub(crate) fn new(configuration: &Configuration) -> Self
	{
		Self
		{
			resourceTemplate: configuration.resourceTemplate(),
			overridesOfResourceTemplate: HashMap::with_capacity(1024),
		}
	}
	
	#[inline(always)]
	pub(crate) fn find(&self, hierarchy: &[String]) -> &HjsonValue
	{
		let mut sliceEndIndex = hierarchy.len();
		loop
		{
			let key = &hierarchy[0 .. sliceEndIndex];
			if let Some(configuration) = self.overridesOfResourceTemplate.get(key)
			{
				return configuration;
			}
			
			if sliceEndIndex == 0
			{
				return &self.resourceTemplate;
			}
			
			sliceEndIndex -= 1;
		}
	}
	
	#[inline(always)]
	pub(crate) fn store(&mut self, hierarchy: Vec<String>, overrideOfResourceTemplate: HjsonValue)
	{
		self.overridesOfResourceTemplate.insert(hierarchy, overrideOfResourceTemplate);
	}
}
