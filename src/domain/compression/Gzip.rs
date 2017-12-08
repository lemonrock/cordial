// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct Gzip
{
	#[serde(default = "Gzip::iterations_default")] iterations: u8,
	#[serde(default = "Gzip::maximum_block_splits_default")] maximum_block_splits: u8,
}

impl Default for Gzip
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			iterations: Self::iterations_default(),
			maximum_block_splits: Self::maximum_block_splits_default(),
		}
	}
}

impl Gzip
{
	#[inline(always)]
	fn iterations_default() -> u8
	{
		15
	}
	
	#[inline(always)]
	fn maximum_block_splits_default() -> u8
	{
		15
	}
	
	//noinspection SpellCheckingInspection
	pub(crate) fn compress(&self, inputData: &[u8]) -> Result<BinaryBody, CordialError>
	{
		warn!("The zopfli library currently does not support options");
		
//		let options = ::zopfli::Options
//		{
//			verbose: false,
//			verbose_more: false,
//			numiterations: if self.iterations == 0
//			{
//				Self::iterations_default() as i32
//			}
//			else
//			{
//				self.iterations as i32
//			},
//			blocksplittingmax: self.maximum_block_splits as i32
//		};
		
		let options = ::zopfli::Options::default();
		
		let mut writer = Vec::with_capacity(inputData.len());
		
		if let Err(error) = ::zopfli::compress(&options, &::zopfli::Format::Gzip, &inputData, &mut writer)
		{
			return Err(CordialError::CouldNotCompressData("gzip", error))
		}
		
		writer.shrink_to_fit();
		
		Ok(BinaryBody(writer))
	}
}
