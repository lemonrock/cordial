// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum BrotliCompressionMode
{
	generic,
	utf8,
	woff,
}

impl Default for BrotliCompressionMode
{
	#[inline(always)]
	fn default() -> Self
	{
		BrotliCompressionMode::utf8
	}
}

impl BrotliCompressionMode
{
	//noinspection SpellCheckingInspection
	fn asBrotliCompressMode(&self) -> ::brotli2::CompressMode
	{
		use self::BrotliCompressionMode::*;
		
		use ::brotli2::CompressMode::*;
		match *self
		{
			generic => Generic,
			utf8 => Text,
			woff => Font,
		}
	}
}
