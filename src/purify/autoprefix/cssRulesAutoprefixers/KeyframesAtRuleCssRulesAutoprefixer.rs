// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// Applies vendor prefixes to @keyframes.
#[derive(Default, Debug, Clone)]
pub struct KeyframesAtRuleCssRulesAutoprefixer
{
	removeUnprefixedAtRule: bool,
	vendorPrefixes: BTreeSet<VendorPrefix>,
}

impl CssRulesAutoprefixer for KeyframesAtRuleCssRulesAutoprefixer
{
	fn autoprefix<C: HasCssRules>(&self, css_rules: &mut C, parent_vendor_prefix: Option<&VendorPrefix>)
	{
		let mut css_rules = css_rules.css_rules_mut();
		
		css_rules.vendor_prefix_at_rules
		(
			self.removeUnprefixedAtRule,
			|cssRule|
			{
				match cssRule
				{
					&CssRule::Keyframes(ref atRule) => Some(atRule),
					_ => None,
				}
			},
			|index, atRule|
			{
				let mut vendorPrefixedCssRules = Vec::with_capacity(self.vendorPrefixes.len());
				for vendorPrefix in self.vendorPrefixes.iter().rev()
				{
					let include = if let Some(parent_vendor_prefix) = parent_vendor_prefix
					{
						parent_vendor_prefix == vendorPrefix
					}
					else
					{
						true
					};
					
					if include
					{
						vendorPrefixedCssRules.push(CssRule::Keyframes(KeyframesAtRule
						{
							vendor_prefix: Some(vendorPrefix.clone()),
							name: atRule.name.clone(),
							keyframes: atRule.keyframes.clone(),
						}));
					}
				}
				vendorPrefixedCssRules
			}
		)
	}
}

impl KeyframesAtRuleCssRulesAutoprefixer
{
	#[inline(always)]
	fn new(can_i_use: &CanIUse, our_rules: &AgentNameAndVersionSet) -> Self
	{
		let mut vendorPrefixes = BTreeSet::new();
		our_rules.prefixes_for_implementations_of_a_feature(can_i_use, &featureName("css-animation"), |agent, version, support|
		{
			if support.requires_prefix()
			{
				vendorPrefixes.insert(mapPrefixToVendorPrefix(agent.prefix(version)));
			}
		});
		
		Self
		{
			removeUnprefixedAtRule: false,
			vendorPrefixes,
		}
	}
}
