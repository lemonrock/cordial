// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use self::cssRulesAutoprefixers::CompositeCssRulesAutoprefixer;
use ::caniuse_serde::*;
use ::caniuse_serde::regional_usage::*;
use ::css::Stylesheet;
use ::css::domain::*;
use ::css::domain::properties::*;
use ::css::domain::selectors::*;
use ::css::domain::VendorPrefix::*;
use ::std::ops::Deref;


pub(crate) mod cssRulesAutoprefixers;
pub(crate) mod propertyDeclarationAutoprefixers;
pub(crate) mod selectorAutoprefixers;


include!("autoprefix_stylesheet.rs");
include!("toFeatureName.rs");
include!("mapPrefixToVendorPrefix.rs");
include!("sensible_rules_to_prefixes.rs");
include!("sensible_rules_to_prefixes_default.rs");
