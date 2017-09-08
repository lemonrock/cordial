// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::futures::future::Either;
use ::futures::future::Empty;
use ::futures::future::Future;
use ::futures::future::FutureResult;
use ::hyper::Body;
use ::hyper::Method;
use ::hyper::header::Headers;
use ::hyper::server::Response;
use ::std::fmt::Debug;
use ::url::Url;


include!("HttpRedirectToHttpsRequestHandler.rs");
include!("HttpsStaticRequestHandler.rs");
include!("RequestHandler.rs");
