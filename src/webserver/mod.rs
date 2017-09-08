// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use self::requestHandlers::*;
use ::futures::FlattenStream;
use ::futures::Future;
use ::futures::Stream;
use ::futures::future::Either;
use ::futures::future::FutureResult;
use ::futures::future::IntoFuture;
use ::futures::future::ok;
use ::hyper::Body;
use ::hyper::Method;
use ::hyper::StatusCode;
use ::hyper::Error::Header as HyperErrorHeader;
use ::hyper::Result as HyperResult;
use ::hyper::header::Allow;
use ::hyper::header::CacheControl;
use ::hyper::header::CacheDirective;
use ::hyper::header::ContentLength;
use ::hyper::header::ContentType;
use ::hyper::header::Date;
use ::hyper::header::Formatter as HeaderFormatter;
use ::hyper::header::Location;
use ::hyper::header::Header;
use ::hyper::header::Headers;
use ::hyper::header::Host;
use ::hyper::header::Raw;
use ::hyper::server::{Http, Service, Request, Response};
use ::ordermap::OrderMap;
use ::rustls::ResolvesServerCert;
use ::rustls::ServerConfig;
use ::rustls::SignatureScheme;
use ::rustls::sign::CertifiedKey;
use ::rustls::sign::RSASigningKey;
use ::rustls::sign::SigningKey;
use ::std::ascii::AsciiExt;
use ::std::borrow::Cow;
use ::std::collections::HashSet;
use ::std::fmt::Result as FormatResult;
use ::std::net::SocketAddr;
use ::std::path::Path;
use ::std::sync::Arc;
use ::std::time::SystemTime;
use ::tokio_io::AsyncRead;
use ::tokio_io::AsyncWrite;
use ::tokio_rustls::ServerConfigExt;
use ::url::percent_encoding::percent_decode;
use ::url::Url;


include!("static_response_only_header.rs");


pub(crate) mod requestHandlers;


include!("commonCacheControlHeader.rs");
include!("CommonResponses.rs");
include!("HttpService.rs");
include!("immutableCacheDirective.rs");
include!("RsaManyServersResolvesServerCert.rs");
include!("Webserver.rs");
include!("X_Content_Type_Options.rs");
include!("X_Frame_Options.rs");
include!("X_XSS_Protection.rs");
