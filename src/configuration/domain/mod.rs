// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::hjson::*;
use ::mktemp::Temp;
use ::serde_hjson::Value as HjsonValue;
use ::std::io::BufReader;
use ::std::io::Read;
use ::std::io::Write;
use ::std::path::*;
use ::std::process::*;
use ::url::Url;


include!("brotli.rs");
include!("BrotliCompressionMode.rs");
include!("compression.rs");
include!("gzip.rs");
include!("ImageTransformation.rs");
include!("InputImageFormat.rs");
include!("language.rs");
include!("localization.rs");
include!("pipeline.rs");
include!("resource.rs");
