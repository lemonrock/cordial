// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use ::std::io::Write;
use ::xml::common::XmlVersion;
use ::xml::name::Name;
use ::xml::namespace::Namespace;
use ::xml::namespace::NS_NO_PREFIX;
use ::xml::writer::EmitterConfig;
use ::xml::writer::Error as EmitterError;
use ::xml::writer::events::XmlEvent;


include!("LengthTrackingWriter.rs");
include!("XmlWriterExt.rs");
include!("XmlWriterResult.rs");
