// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct brotli
{
	compressionMode: BrotliCompressionMode,
}

impl brotli
{
	// NOTE: This approach is probably not optimal; it should read-and-write in blocks of, say 128Kb, not read all the file contents at once (`fileContentsAsBytes()`)
	// NOTE: That said, most files are likely to be quite small that use this compressor
	pub fn compress(&self, inputFilePath: &Path, inData: &[u8]) -> Result<(), CordialError>
	{
		let outputFilePath = inputFilePath.appendExtension("gz");
		
		let mut compressionParameters = ::brotli2::CompressParams::new();
		compressionParameters.mode(self.compressionMode.asBrotliCompressMode()).quality(11).lgwin(24).lgblock(24);
		
		{
			let writer = File::create(&outputFilePath).context(&outputFilePath)?;
			let mut compressor = ::brotli2::write::BrotliEncoder::from_params(writer, &compressionParameters);
			compressor.write_all(&inData).context(inputFilePath)?;
			compressor.finish().context(inputFilePath)?;
		}
		
		Ok(())
	}
}
