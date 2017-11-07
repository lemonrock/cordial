// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// Applies vendor prefixes to @viewport.
#[derive(Default, Debug, Clone)]
pub struct ViewportAtRuleCssRulesAutoprefixer
{
	removeUnprefixedAtRule: bool,
	vendorPrefixesAndPropertiesToRetain: BTreeMap<VendorPrefix, Option<HashSet<String>>>,
}

impl CssRulesAutoprefixer for ViewportAtRuleCssRulesAutoprefixer
{
	fn autoprefix<C: HasCssRules>(&self, css_rules: &mut C, parent_vendor_prefix: Option<&VendorPrefix>)
	{
		let css_rules = css_rules.css_rules_mut();
		
		css_rules.vendor_prefix_at_rules
		(
			self.removeUnprefixedAtRule,
			|cssRule|
			{
				match cssRule
				{
					&CssRule::Viewport(ref atRule) => Some(atRule),
					_ => None,
				}
			},
			|_index, atRule|
			{
				let mut vendorPrefixedCssRules = Vec::with_capacity(self.vendorPrefixesAndPropertiesToRetain.len());
				for (vendorPrefix, validCssNames) in self.vendorPrefixesAndPropertiesToRetain.iter().rev()
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
						vendorPrefixedCssRules.push(Self::createVendorPrefixedAtRule(atRule, vendorPrefix, validCssNames.as_ref()));
					}
				}
				vendorPrefixedCssRules
			}
		)
	}
}

impl ViewportAtRuleCssRulesAutoprefixer
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn new(can_i_use: &CanIUse, our_rules: &AgentNameAndVersionSet) -> Self
	{
		let mut vendorPrefixesAndPropertiesToRetain = BTreeMap::new();
		
		our_rules.support_for_a_feature(can_i_use, &toFeatureName("css-deviceadaptation"), |agent, version, support|
		{
			if support.requires_prefix()
			{
				let vendorPrefix = mapPrefixToVendorPrefix(agent.prefix(version));
				
				let map = match vendorPrefix
				{
					o => Some(hashset!
					{
						"orientation".to_owned()
					}),
					
					ms => Some(hashset!
					{
						"height".to_owned(),
						"max-height".to_owned(),
						"min-height".to_owned(),
						"width".to_owned(),
						"max-width".to_owned(),
						"min-width".to_owned(),
					}),
					
					_ => None,
				};
				
				vendorPrefixesAndPropertiesToRetain.insert(vendorPrefix, map);
			}
		});
		
		Self
		{
			removeUnprefixedAtRule: true,
			vendorPrefixesAndPropertiesToRetain,
		}
	}
	
	#[inline(always)]
	fn createVendorPrefixedAtRule(viewportAtRule: &ViewportAtRule, vendorPrefix: &VendorPrefix, validCssNames: Option<&HashSet<String>>) -> CssRule
	{
		CssRule::Viewport
		(
			ViewportAtRule
			{
				vendor_prefix: Some(vendorPrefix.clone()),
				declarations: match validCssNames
				{
					Some(validCssNames) =>
					{
						let mut declarations = Vec::with_capacity(validCssNames.len());
						
						for declaration in viewportAtRule.declarations.iter()
						{
							let css_name = declaration.descriptor.css_name();
							if validCssNames.contains(css_name)
							{
								declarations.push(declaration.clone());
							}
						}
						
						declarations
					},
					None =>
					{
						let mut declarations = Vec::with_capacity(viewportAtRule.declarations.len());
						
						for declaration in viewportAtRule.declarations.iter()
						{
							declarations.push(declaration.clone());
						}
						
						declarations
					},
				},
			}
		)
	}
}
