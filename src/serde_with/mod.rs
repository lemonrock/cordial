// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::serde::de::Deserialize;
use ::serde::de::Deserializer;
use ::serde::de::Error as DeserializeError;
use ::serde::de::Visitor;
use ::serde::ser::Serializer;
use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::ops::DerefMut;


pub(crate) mod ContentTypeSerde;
pub(crate) mod HttpDateSerde;
pub(crate) mod StatusCodeSerde;


include!("MimeSerde.rs");
include!("UrlSerde.rs");
