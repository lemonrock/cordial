// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct compression
{
	gzip: gzip,
	brotli: brotli,
}

impl compression
{
	#[inline(always)]
	pub fn compress(&self, inputData: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CordialError>
	{
		let gzipCompressed = self.gzip.compress(&inputData)?;
		let brotliCompressed = self.brotli.compress(&inputData)?;
		
		Ok((gzipCompressed, brotliCompressed))
	}
}
