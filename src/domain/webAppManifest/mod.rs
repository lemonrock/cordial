// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::serde::ser::Error as SerializeError;
use ::serde::ser::SerializeStruct;
use ::std::convert::AsRef;
use ::std::convert::Into;
use ::std::cmp::*;
use ::std::hash::*;


include!("WebAddManifestJsonRoot.rs");
include!("WebAddManifestSerializationState.rs");
include!("WebAppManifestAbstract.rs");
include!("WebAppManifestIcon.rs");
include!("WebAppManifestIconPixelDensity.rs");
include!("WebAppManifestOrientation.rs");
