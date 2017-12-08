// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct MimeSerde(pub Mime);

impl Deref for MimeSerde
{
	type Target = Mime;
	
	fn deref(&self) -> &Mime
	{
		&self.0
	}
}

impl DerefMut for MimeSerde
{
	fn deref_mut(&mut self) -> &mut Mime
	{
		&mut self.0
	}
}

impl<'de> Deserialize<'de> for MimeSerde
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct MimeVisitor;
		
		impl<'de> Visitor<'de> for MimeVisitor
		{
			type Value = Mime;
			
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("MIME string")
			}
			
			fn visit_str<E: DeserializeError>(self, value: &str) -> Result<Self::Value, E>
			{
				match value.parse()
				{
					Err(error) => Err(E::custom(format!("Could not parse MIME string: {:?}", error))),
					Ok(mime) => Ok(mime)
				}
			}
			
			fn visit_borrowed_str<E: DeserializeError>(self, value: &'de str) -> Result<Self::Value, E>
			{
				match value.parse()
				{
					Err(error) => Err(E::custom(format!("Could not parse MIME string: {:?}", error))),
					Ok(mime) => Ok(mime)
				}
			}
			
			fn visit_string<E: DeserializeError>(self, value: String) -> Result<Self::Value, E>
			{
				match value.as_str().parse()
				{
					Err(error) => Err(E::custom(format!("Could not parse MIME string: {:?}", error))),
					Ok(mime) => Ok(mime)
				}
			}
		}
		
		deserializer.deserialize_any(MimeVisitor).map(|mime| MimeSerde(mime))
	}
}
