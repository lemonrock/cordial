// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// Descends through CSS rules and auto-prefixes
#[derive(Debug, Clone)]
pub struct DescendingCssRulesAutoprefixer
{
	selectorAutoprefixer: CompositeSelectorAutoprefixer,
	propertyDeclarationAutoprefixer: CompositePropertyDeclarationAutoprefixer,
}

impl CssRulesAutoprefixer for DescendingCssRulesAutoprefixer
{
	fn autoprefix<C: HasCssRules>(&self, css_rules: &mut C, parent_vendor_prefix: Option<&VendorPrefix>)
	{
		let mut css_rules = css_rules.css_rules_mut();
		
		let mut index = 0;
		while index < css_rules.0.len()
		{
			let cssRule = unsafe { &css_rules.0.get_unchecked_mut(index) };
			match cssRule
			{
				&mut CssRule::Style(ref mut styleRule) =>
				{
					self.selectorAutoprefixer.autoprefix(&mut styleRule.selectors, parent_vendor_prefix);
					self.propertyDeclarationAutoprefixer.autoprefix(styleRule, parent_vendor_prefix);
				}
				
				&mut CssRule::Keyframe(ref mut keyframesAtRule) =>
				{
					let parent_vendor_prefix = keyframesAtRule.vendor_prefix.as_ref();
					for keyframe in keyframesAtRule.keyframes.iter_mut()
					{
						self.propertyDeclarationAutoprefixer.autoprefix(keyframe, parent_vendor_prefix);
					}
				}
				
				&mut CssRule::Page(ref mut pageAtRule) =>
				{
					self.propertyDeclarationAutoprefixer.autoprefix(pageAtRule, None);
				}
				
				&mut CssRule::Document(ref mut documentAtRule) =>
				{
					let parent_vendor_prefix = documentAtRule.vendor_prefix.as_ref();
					self.autoprefix(documentAtRule, parent_vendor_prefix);
				}
				
				&mut CssRule::Supports(ref mut supportsAtRule) =>
				{
					self.autoprefix(supportsAtRule, parent_vendor_prefix);
				}
				
				&mut CssRule::Media(ref mut mediaAtRule) =>
				{
					self.autoprefix(supportsAtRule, parent_vendor_prefix);
				}
			}
			index +=1;
		}
	}
}

impl DescendingCssRulesAutoprefixer
{
	#[inline(always)]
	fn new(can_i_use: &CanIUse, our_rules: &AgentNameAndVersionSet) -> Self
	{
		Self
		{
			selectorAutoprefixer: CompositeSelectorAutoprefixer::new(can_i_use, our_rules),
			propertyDeclarationAutoprefixer: CompositePropertyDeclarationAutoprefixer::new(can_i_use, our_rules),
		}
	}
}
