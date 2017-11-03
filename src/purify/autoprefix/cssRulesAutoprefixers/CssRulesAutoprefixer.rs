// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// Applies vendor prefixes to CSS rules, eg @document, @viewport, etc.
pub trait CssRulesAutoprefixer
{
	/// Applies vendor prefixes to CSS rules, eg @document, @viewport, etc.
	fn autoprefix(&self, css_rules: &mut CssRules, parent_vendor_prefix: Option<&VendorPrefix>);
}
