// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct Brotli
{
	#[serde(default)] compression_mode: BrotliCompressionMode,
}

impl Default for Brotli
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			compression_mode: BrotliCompressionMode::default(),
		}
	}
}

impl Brotli
{
	pub(crate) fn compress(&self, inputData: &[u8]) -> Result<BinaryBody, CordialError>
	{
		let mut compressionParameters = ::brotli2::CompressParams::new();
		compressionParameters.mode(self.compression_mode.asBrotliCompressMode()).quality(11).lgwin(24).lgblock(24);
		
		let mut writer = Vec::with_capacity(inputData.len());
		{
			let mut compressor = ::brotli2::write::BrotliEncoder::from_params(writer.as_mut_slice(), &compressionParameters);
			
			if let Err(error) = compressor.write_all(&inputData)
			{
				return Err(CordialError::CouldNotCompressData("brotli write_all", error))
			}
			
			if let Err(error) = compressor.finish()
			{
				return Err(CordialError::CouldNotCompressData("brotli finish", error))
			}
		}
		writer.shrink_to_fit();
		
		Ok(BinaryBody(writer))
	}
}
