// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::hyper::header::HttpDate;


#[inline(always)]
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<HttpDate, D::Error>
{
	struct HttpDateVisitor;
	
	impl<'de> Visitor<'de> for HttpDateVisitor
	{
		type Value = HttpDate;
		
		fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
		{
			formatter.write_str("an HttpDate string")
		}
		
		fn visit_str<E: DeserializeError>(self, value: &str) -> Result<Self::Value, E>
		{
			value.parse().map_err(|_| E::custom("Invalid HttpDate"))
		}
	}
	
	deserializer.deserialize_str(HttpDateVisitor)
}

#[inline(always)]
pub(crate) fn serialize<S: Serializer>(value: &HttpDate, serializer: S) -> Result<S::Ok, S::Error>
{
	// Internally, HttpDate formats for display as RFC 822 format. It does not provide any public methods to access its inner parsed time value...
	let string = format!("{}", value);
	serializer.serialize_str(&string)
}
