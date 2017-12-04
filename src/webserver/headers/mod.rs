// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use ::hyper::Error::Header as HyperErrorHeader;
use ::hyper::Result as HyperResult;
use ::hyper::header::CacheControl;
use ::hyper::header::CacheDirective;
use ::hyper::header::Formatter as HeaderFormatter;
use ::hyper::header::Header;
use ::hyper::header::Raw;
use ::std::fmt::Result as FormatResult;


include!("static_response_only_header.rs");


include!("commonCacheControlHeader.rs");
include!("immutableCacheDirective.rs");
include!("Strict_Transport_Security.rs");
include!("X_Content_Type_Options.rs");
include!("X_Frame_Options.rs");
include!("X_Robots_Tag.rs");
include!("X_Robots_Tag_Data.rs");
include!("X_XSS_Protection.rs");
