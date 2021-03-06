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
use ::hyper::header::AccessControlAllowOrigin;
use ::hyper::header::AccessControlRequestHeaders;
use ::hyper::header::AccessControlRequestMethod;
use ::hyper::header::ByteRangeSpec;
use ::hyper::header::ContentEncoding;
use ::hyper::header::Encoding;
use ::hyper::header::ETag;
use ::hyper::header::EntityTag;
use ::hyper::header::Headers;
use ::hyper::header::HttpDate;
use ::hyper::header::IfMatch;
use ::hyper::header::IfModifiedSince;
use ::hyper::header::IfNoneMatch;
use ::hyper::header::IfRange;
use ::hyper::header::IfUnmodifiedSince;
use ::hyper::header::Origin;
use ::hyper::header::Range;
use ::hyper::header::RangeUnit;
use ::hyper::mime::*;
use ::hyper::server::Response;
use ::mime_multipart::Node;
use ::mime_multipart::Part;
use ::mime_multipart::generate_boundary;
use ::mime_multipart::write_multipart;
use ::radix_trie::Trie;
use ::radix_trie::TrieCommon;
use ::ring::digest::Context;
use ::ring::digest::SHA256;
use ::std::convert::AsRef;
use ::std::collections::BTreeMap;
use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::std::time::SystemTime;
use ::std::ops::Deref;
use ::url::Url;
use ::zero85::ToZ85;


include!("BinaryBody.rs");
include!("HttpRedirectToHttpsRequestHandler.rs");
include!("HttpsStaticRequestHandler.rs");
include!("PreferredCompression.rs");
include!("RegularAndPjaxStaticResponse.rs");
include!("RequestHandler.rs");
include!("ResponseBody.rs");
include!("ResponseHeaders.rs");
include!("Responses.rs");
include!("StaticResponse.rs");
include!("StaticResponseVersions.rs");
include!("Utf8Body.rs");
