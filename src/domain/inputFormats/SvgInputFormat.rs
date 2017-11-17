// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) enum SvgInputFormat
{
	SVG,
	
	SVGBOB,
	
	MON_ARTIST(MonArtist),
	
	QR_CODE(QrCodeData),

	MEME(Meme),

	BARCODE(Barcode),
}

impl Default for SvgInputFormat
{
	#[inline(always)]
	fn default() -> Self
	{
		SvgInputFormat::SVG
	}
}

impl InputFormat for SvgInputFormat
{
	#[inline(always)]
	fn fileExtensions(&self) -> &'static [&'static str]
	{
		use self::SvgInputFormat::*;
		
		match *self
		{
			SVG => &[".svg"],
			
			SVGBOB => &[".svgbob"],
			
			MON_ARTIST(_) => &[".mon-artist"],
			
			QR_CODE(_) => &[".qr-code"],
			
			MEME(_) => &[".meme"],
			
			BARCODE(_) => &[".barcode"],
		}
	}
	
	#[inline(always)]
	fn allFileExtensions() -> &'static [&'static str]
	{
		&[
			".svg",
			".svgbob",
			".mon-artist",
			".qr-code",
			".meme",
			".barcode",
		]
	}
}

impl SvgInputFormat
{
	#[inline(always)]
	pub(crate) fn svgString(&self, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, configuration: &Configuration) -> Result<String, CordialError>
	{
		use self::SvgInputFormat::*;
		
		match *self
		{
			SVG => Ok(inputContentFilePath.fileContentsAsString().context(inputContentFilePath)?),
			
			SVGBOB =>
			{
				let bobString = inputContentFilePath.fileContentsAsString().context(inputContentFilePath)?;
				Ok(MarkdownPlugin::svgbobFromStr(&bobString, false))
			}
			
			MON_ARTIST(ref monArtist) => monArtist.svgString(inputContentFilePath, resourceUrl, configuration),
			
			QR_CODE(ref qrCodeData) => qrCodeData.svgString(),
			
			MEME(ref meme) => meme.svgString(inputContentFilePath),
			
			BARCODE(ref barcode) => barcode.svgString(),
		}
	}
}
