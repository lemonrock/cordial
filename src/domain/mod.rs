// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use super::hjson::*;
use super::woff::*;
use super::xmlExtra::*;
use super::webserver::RsaManyServersResolvesServerCert;
use super::webserver::UpdatableTlsServerConfigurationFactory;
use super::webserver::Webserver;
use super::webserver::requestHandlers::*;
use super::webserver::requestHandlerFactories::*;
use self::compression::*;
use self::daemon::*;
use self::engiffen::*;
use self::html::*;
use self::markdown::*;
use self::people::*;
use self::pipelines::*;
use self::robots::*;
use self::rss::*;
use self::siteMap::*;
use self::webAppManifest::*;
use ::base64::encode_config as base64Encode;
use ::base64::URL_SAFE_NO_PAD;
use ::css_autoprefix::autoprefix_stylesheet;
use ::css_autoprefix::caniuse_serde::sensible_choices;
use ::css_autoprefix::caniuse_serde::UsagePercentage;
use ::css_autoprefix::caniuse_serde::regional_usage::RegionalUsage;
use ::css_autoprefix::caniuse_serde::regional_usage::RegionalUsages;
use ::css_autoprefix::css::Stylesheet;
use ::chrono::DateTime;
use ::chrono::Utc;
use ::handlebars::Handlebars;
use ::hyper::StatusCode;
use ::hyper::header::ContentType;
use ::hyper::mime::Mime;
use ::hyper::mime::TEXT_CSS;
use ::mktemp::Temp;
use ::ordermap::OrderMap;
use ::rustls::ServerConfig;
use ::rustls::ServerSessionMemoryCache;
use ::serde::de::Error as DeserializeError;
use ::serde::de::Deserialize;
use ::serde::de::Deserializer;
use ::serde::de::Visitor;
use ::serde::ser::Serialize;
use ::serde::ser::Serializer;
use ::serde_hjson::Map as HjsonMap;
use ::serde_hjson::Value as HjsonValue;
use ::std::borrow::Cow;
use ::std::cell::Cell;
use ::std::cell::Ref;
use ::std::cell::RefCell;
use ::std::collections::BTreeMap;
use ::std::collections::BTreeSet;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::fmt;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fs::File;
use ::std::io::BufReader;
use ::std::io::Write;
use ::std::marker::PhantomData;
use ::std::mem::transmute;
use ::std::net::*;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::path::Component::Normal;
use ::std::rc::Rc;
use ::std::sync::Arc;
use ::std::time::Duration;
use ::std::time::SystemTime;
use ::std::time::UNIX_EPOCH;
use ::url::Url;
use ::url::percent_encoding::USERINFO_ENCODE_SET;
use ::url::percent_encoding::utf8_percent_encode;


pub(crate) mod compression;
pub(crate) mod configure;
pub(crate) mod daemon;
pub(crate) mod engiffen;
pub(crate) mod html;
pub(crate) mod images;
pub(crate) mod inputFormats;
pub(crate) mod localization;
pub(crate) mod markdown;
pub(crate) mod people;
pub(crate) mod pipelines;
pub(crate) mod robots;
pub(crate) mod rss;
pub(crate) mod siteMap;
pub(crate) mod svg;
pub(crate) mod webAppManifest;


include!("HandlebarsTemplateRenderer.rs");
include!("HandlebarsWrapper.rs");
include!("HeaderGenerator.rs");
include!("MimeNewType.rs");
include!("mimeType.rs");
include!("ProcessingPriority.rs");
include!("Resource.rs");
include!("ResourcePipeline.rs");
include!("Resources.rs");
include!("ResourceReference.rs");
include!("ResourceTag.rs");
include!("ResourceUrl.rs");
include!("UrlData.rs");
include!("UrlDataDetails.rs");
include!("UrlSerde.rs");
