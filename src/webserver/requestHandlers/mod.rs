// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::futures::future::Either;
use ::futures::future::Empty;
use ::futures::future::Future;
use ::futures::future::FutureResult;
use ::hyper::Body;
use ::hyper::Method;
use ::hyper::header::AcceptEncoding;
use ::hyper::header::ContentEncoding;
use ::hyper::header::Encoding;
use ::hyper::header::ETag;
use ::hyper::header::EntityTag;
use ::hyper::header::Headers;
use ::hyper::server::Response;
use ::radix_trie::Trie;
use ::ring::digest::Context;
use ::ring::digest::SHA256;
use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::url::Url;
use ::zero85::ToZ85;


include!("HttpRedirectToHttpsRequestHandler.rs");
include!("HttpsStaticRequestHandler.rs");
include!("PreferredEncoding.rs");
include!("RequestHandler.rs");
include!("RegularAndPjaxStaticResponse.rs");
include!("Resources.rs");
include!("StaticResponse.rs");
include!("StaticResponseVersions.rs");
