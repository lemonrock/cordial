// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::serde::de::DeserializeOwned;
use ::serde_hjson::from_reader as hjsonFromReader;
use ::serde_hjson::Value as HjsonValue;
use ::serde_json::from_value as jsonFromValue;
use ::std::cmp::max;
use ::std::cmp::min;
use ::std::fs::File;


include!("ArrayMergeStrategy.rs");
include!("deserializeHjson.rs");
include!("hjsonMerge.rs");
include!("hjsonToJsonBecauseHjsonCrateUsesAnOldVersionOfSerde.rs");
include!("loadHjson.rs");
include!("loadHjsonIfExtant.rs");
include!("loadHjsonIfExtantAndMerge.rs");
