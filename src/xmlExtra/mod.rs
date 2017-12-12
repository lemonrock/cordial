// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::CordialError;
use super::domain::UrlData;
use super::domain::people::EMailAddress;
use super::domain::localization::Iso639Dash1Alpha2Language;
use ::chrono::DateTime;
use ::chrono::Utc;
use ::std::borrow::Cow;
use ::std::cell::Cell;
use ::std::io;
use ::std::io::Write;
use ::url::Url;
use ::xml::ParserConfig;
use ::xml::attribute::Attribute;
use ::xml::attribute::OwnedAttribute;
use ::xml::common::XmlVersion;
use ::xml::name::Name;
use ::xml::namespace::Namespace;
use ::xml::reader::XmlEvent as ReaderXmlEvent;
use ::xml::writer::EmitterConfig;
use ::xml::writer::EventWriter;
use ::xml::writer::events::XmlEvent;


include!("EventWriterExt.rs");
include!("LengthTrackingWriter.rs");
include!("minifyingEmitterConfig.rs");
include!("minifyingVecEventWriter.rs");
include!("minifyXml.rs");
include!("XmlAttributeExt.rs");
include!("XmlNameExt.rs");
