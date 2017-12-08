// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::hyper::header::ContentType;


#[inline(always)]
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<ContentType, D::Error>
{
	struct ContentTypeVisitor;
	
	impl<'de> Visitor<'de> for ContentTypeVisitor
	{
		type Value = ContentType;
		
		fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
		{
			formatter.write_str("a MIME content type string")
		}
		
		fn visit_str<E: DeserializeError>(self, value: &str) -> Result<Self::Value, E>
		{
			match value.parse()
			{
				Err(_) => Err(E::custom("Invalid content type")),
				Ok(mime) => Ok(ContentType(mime)),
			}
		}
	}
	
	deserializer.deserialize_str(ContentTypeVisitor)
}

#[inline(always)]
pub(crate) fn serialize<S: Serializer>(value: &ContentType, serializer: S) -> Result<S::Ok, S::Error>
{
	serializer.serialize_str(value.0.as_ref())
}
