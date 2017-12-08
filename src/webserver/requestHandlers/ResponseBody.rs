// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum ResponseBody
{
	Utf8(Utf8Body),
	Binary(BinaryBody)
}

impl AsRef<[u8]> for ResponseBody
{
	fn as_ref(&self) -> &[u8]
	{
		use self::ResponseBody::*;
		
		match *self
		{
			Utf8(ref body) => body.as_ref(),
			Binary(ref body) => body.as_ref(),
		}
	}
}

impl AsRef<Vec<u8>> for ResponseBody
{
	fn as_ref(&self) -> &Vec<u8>
	{
		use self::ResponseBody::*;
		
		match *self
		{
			Utf8(ref body) => body.deref(),
			Binary(ref body) => body.deref(),
		}
	}
}

impl Deref for ResponseBody
{
	type Target = Vec<u8>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		use self::ResponseBody::*;
		
		match *self
		{
			Utf8(ref body) => body.deref(),
			Binary(ref body) => body.deref(),
		}
	}
}

impl ResponseBody
{
	#[inline(always)]
	pub(crate) fn utf8(body: Vec<u8>) -> Self
	{
		ResponseBody::Utf8(Utf8Body(body))
	}
	
	#[inline(always)]
	pub(crate) fn binary(body: Vec<u8>) -> Self
	{
		ResponseBody::Binary(BinaryBody(body))
	}
}
