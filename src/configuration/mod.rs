// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of cordial, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use super::hjson::*;
use ::serde_hjson::Map as HjsonMap;
use ::serde_hjson::Value as HjsonValue;
use ::std::collections::HashMap;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::path::Component::Normal;

pub(crate) mod domain;


include!("Configuration.rs");
include!("DiscoverResources.rs");
include!("ResourceTemplates.rs");
