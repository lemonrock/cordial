// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct FacebookOpenGraphBusinessHours
{
	open: NaiveTime,
	close: NaiveTime,
}

impl FacebookOpenGraphBusinessHours
{
	#[inline(always)]
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>) -> Result<(), CordialError>
	{
		if self.close <= self.open
		{
			return Err(CordialError::Configuration("Closing time can not be the same or before opening time".to_owned()));
		}
		
		endHeadNodes.push(meta_with_property_and_content("business:hours:start", &self.open.format("%H:%M").to_string()));
		endHeadNodes.push(meta_with_property_and_content("business:hours:end", &self.close.format("%H:%M").to_string()));
		
		Ok(())
	}
}
