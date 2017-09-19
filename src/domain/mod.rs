// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use super::hjson::*;
use super::xmlExtra::*;
use super::webserver::RsaManyServersResolvesServerCert;
use super::webserver::UpdatableTlsServerConfigurationFactory;
use super::webserver::Webserver;
use super::webserver::requestHandlers::*;
use super::webserver::requestHandlerFactories::*;
use ::base64::encode_config as base64Encode;
use ::base64::URL_SAFE_NO_PAD;
use ::chrono::DateTime;
use ::chrono::Utc;
use ::daemonize::Daemonize;
use ::daemonize::Group;
use ::daemonize::User;
use ::handlebars::Handlebars;
use ::hyper::StatusCode;
use ::hyper::header::ContentType;
use ::hyper::mime::TEXT_CSS;
use ::image::GenericImage;
use ::mktemp::Temp;
use ::net2::TcpBuilder;
use ::net2::TcpListenerExt;
use ::net2::unix::UnixTcpBuilderExt;
use ::ordermap::OrderMap;
use ::rustls::ServerConfig;
use ::rustls::ServerSessionMemoryCache;
use ::serde::de::Error as DeserializeError;
use ::serde::de::Deserialize;
use ::serde::de::Deserializer;
use ::serde::de::Visitor;
use ::serde_hjson::Map as HjsonMap;
use ::serde_hjson::Value as HjsonValue;
use ::std::ascii::AsciiExt;
use ::std::borrow::Cow;
use ::std::cell::Cell;
use ::std::collections::BTreeMap;
use ::std::collections::BTreeSet;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::io::Write;
use ::std::marker::PhantomData;
use ::std::mem::transmute;
use ::std::net::*;
use ::std::os::unix::fs::FileTypeExt;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::path::Component::Normal;
use ::std::sync::Arc;
use ::std::time::Duration;
use ::std::time::SystemTime;
use ::std::time::UNIX_EPOCH;
use ::url::Url;
use ::url::percent_encoding::USERINFO_ENCODE_SET;
use ::url::percent_encoding::utf8_percent_encode;
use ::xml::attribute::Attribute;
use ::xml::name::Name;
use ::xml::namespace::Namespace;
use ::xml::namespace::NS_NO_PREFIX;
use ::xml::writer::EmitterConfig;
use ::xml::writer::EventWriter;


include!("Abstract.rs");
include!("Brotli.rs");
include!("BrotliCompressionMode.rs");
include!("Compression.rs");
include!("Configuration.rs");
include!("Daemon.rs");
include!("DiscoverResources.rs");
include!("FromStringOrNumber.rs");
include!("generateHeaders.rs");
include!("GroupNewType.rs");
include!("Gzip.rs");
include!("HtmlVariant.rs");
include!("ImageAbstract.rs");
include!("ImageCrop.rs");
include!("ImageScale.rs");
include!("ImageSourceSet.rs");
include!("ImageSourceSetEntry.rs");
include!("ImageTransformation.rs");
include!("InputImageFormat.rs");
include!("Language.rs");
include!("Localization.rs");
include!("Pipeline.rs");
include!("ProcessingPriority.rs");
include!("Resource.rs");
include!("RobotDirective.rs");
include!("RobotGroup.rs");
include!("RobotsTxt.rs");
include!("ResourceTemplates.rs");
include!("ServerSocket.rs");
include!("Settings.rs");
include!("SiteMap.rs");
include!("SiteMapChangeFrequency.rs");
include!("SiteMapPriority.rs");
include!("SiteMapWebPage.rs");
include!("SiteMapWebPageImage.rs");
include!("StringOrNumberVisitor.rs");
include!("TransformFilterType.rs");
include!("UserNewType.rs");
