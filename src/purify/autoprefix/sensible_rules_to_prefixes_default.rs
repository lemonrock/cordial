// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// Obtain (CanIUse, AgentNameAndVersionSet) for `autoprefix_stylesheet()`
#[inline(always)]
pub fn sensible_rules_to_prefixes_default() -> (CanIUse, AgentNameAndVersionSet)
{
	let maximum_release_age_from_can_i_use_database_last_updated = 54 + 12; // Firefox ESR release cycle + 12 weeks (2x cycles overlap)
	let minimum_usage_threshold = UsagePercentage::OnePerMille;
	let regional_usages = vec!
	[
		Asia.deref(),
		Europe.deref(),
		NorthAmerica.deref(),
		SouthAmerica.deref(),
		AU.deref(),
		NZ.deref(),
	];
	sensible_rules_to_prefixes(maximum_release_age_from_can_i_use_database_last_updated, minimum_usage_threshold, &regional_usages)
}
