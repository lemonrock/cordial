// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use super::browserSets::BrowserSet;
use ::css::domain::CssRule;
use ::css::domain::CssRules;
use ::css::domain::VendorPrefix;
use ::css::domain::atRules::document::DocumentAtRule;
use ::css::domain::atRules::keyframes::KeyframesAtRule;
use ::css::domain::atRules::viewport::ViewportAtRule;
use ::std::collections::BTreeMap;
use ::std::collections::BTreeSet;
use ::std::collections::HashSet;


include!("CompositeCssRulesAutoprefixer.rs");
include!("CssRulesAutoprefixer.rs");
include!("DescendingCssRulesAutoprefixer.rs");
include!("DocumentAtRuleCssRulesAutoprefixer.rs");
include!("KeyframesAtRuleCssRulesAutoprefixer.rs");
include!("ViewportAtRuleCssRulesAutoprefixer.rs");
