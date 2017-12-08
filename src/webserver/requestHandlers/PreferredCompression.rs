// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum PreferredCompression
{
	uncompressed,
	gzip,
	brotli,
}

impl PreferredCompression
{
	/// NOTE: This algorithm completely ignores quality parameter weights, q=0 and forms such as `*;q=0`
	#[inline(always)]
	pub(crate) fn preferredEncoding(acceptEncoding: Option<&AcceptEncoding>) -> Self
	{
		use self::PreferredCompression::*;
		
		match acceptEncoding
		{
			None => uncompressed,
			Some(acceptEncoding) =>
			{
				//NOTE: This implementation ignores quality weights, including q=0 and also '*' types
				let mut supportsGzip = false;
				for qualityItem in acceptEncoding.0.iter()
				{
					use ::hyper::header::Encoding::*;
					match qualityItem.item
					{
						Brotli => return brotli,
						Gzip => supportsGzip = true,
						_ => (),
					}
				}
				
				if supportsGzip
				{
					gzip
				}
				else
				{
					uncompressed
				}
			}
		}
	}
}
