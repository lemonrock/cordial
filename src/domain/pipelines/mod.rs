// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use super::markdown::*;
use super::super::webserver::headers::commonCacheControlHeader;
use self::ProcessingPriority::*;
use self::ResourceTag::*;
use ::css_purify::html5ever_ext::*;
use ::serde::de;


include!("CssPipeline.rs");
include!("GifAnimationPipeline.rs");
include!("FontPipeline.rs");
include!("HtmlPipeline.rs");
include!("is_downloadable_false_default.rs");
include!("is_versioned_true_default.rs");
include!("max_age_in_seconds_long_default.rs");
include!("mimeType.rs");
include!("Pipeline.rs");
include!("RasterImagePipeline.rs");
include!("RawPipeline.rs");
include!("SvgPipeline.rs");
