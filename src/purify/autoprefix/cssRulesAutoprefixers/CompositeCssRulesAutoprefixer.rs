// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// Combines all known rules to auto-prefix a CSS stylesheet
#[derive(Debug, Clone)]
pub struct CompositeCssRulesAutoprefixer
{
	documentAtRuleCssRulesAutoprefixer: DocumentAtRuleCssRulesAutoprefixer,
	keyframesAtRuleCssRulesAutoprefixer: KeyframesAtRuleCssRulesAutoprefixer,
	viewportAtRuleCssRulesAutoprefixer: ViewportAtRuleCssRulesAutoprefixer,
	descendingCssRulesAutoprefixer: DescendingCssRulesAutoprefixer,
}

impl CssRulesAutoprefixer for CompositeCssRulesAutoprefixer
{
	#[inline(always)]
	fn autoprefix<C: HasCssRules>(&self, css_rules: &mut C, parent_vendor_prefix: Option<&VendorPrefix>)
	{
		self.documentAtRuleCssRulesAutoprefixer.autoprefix(css_rules, parent_vendor_prefix);
		self.keyframesAtRuleCssRulesAutoprefixer.autoprefix(css_rules, parent_vendor_prefix);
		self.viewportAtRuleCssRulesAutoprefixer.autoprefix(css_rules, parent_vendor_prefix);
		self.descendingCssRulesAutoprefixer.autoprefix(css_rules, parent_vendor_prefix);
	}
}

impl CompositeCssRulesAutoprefixer
{
	/// Use this method to autoprefix an entire stylesheet
	#[inline(always)]
	pub fn autoprefix_stylesheet(&self, stylesheet: &mut Stylesheet)
	{
		self.autoprefix(stylesheet, None)
	}
	
	/// Use this method to obtain an auto-prefixer suitable for a stylesheet
	#[inline(always)]
	pub fn new(can_i_use: &CanIUse, our_rules: &AgentNameAndVersionSet) -> Self
	{
		Self
		{
			documentAtRuleCssRulesAutoprefixer: DocumentAtRuleCssRulesAutoprefixer::new(can_i_use, our_rules),
			keyframesAtRuleCssRulesAutoprefixer: KeyframesAtRuleCssRulesAutoprefixer::new(can_i_use, our_rules),
			viewportAtRuleCssRulesAutoprefixer: ViewportAtRuleCssRulesAutoprefixer::new(can_i_use, our_rules),
			descendingCssRulesAutoprefixer: DescendingCssRulesAutoprefixer::new(can_i_use, our_rules)
		}
	}
}
