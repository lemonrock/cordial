// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::xml::attribute::Attribute as XmlAttribute;
use ::xml::namespace::Namespace;
use ::xml::namespace::NS_NO_PREFIX;
use ::xml::writer::EmitterConfig;
use ::xml::writer::EventWriter;


include!("SiteMap.rs");
include!("SiteMapChangeFrequency.rs");
include!("SiteMapPriority.rs");
include!("SiteMapWebPage.rs");
include!("SiteMapWebPageAudioVideo.rs");
include!("SiteMapWebPageImage.rs");
