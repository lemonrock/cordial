// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// https://github.com/buntine/barcoders
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct Barcode
{
	#[serde(default)] variant: BarcodeVariant,
	data: String,
	height: u32,
	narrow_bar_width: u32,
	#[serde(default = "Barcode::foreground_color_default")] foreground_color: [u8; 4],
	#[serde(default = "Barcode::background_color_default")] background_color: [u8; 4],
}

impl Barcode
{
	pub(crate) fn svgString(&self) -> Result<String, CordialError>
	{
		use ::barcoders::generators::svg::Color as BarcodeSvgColor;
		use ::barcoders::generators::svg::SVG as BarcodeSvg;
		use ::barcoders::sym;
		
		let barcodeSvg = BarcodeSvg
		{
			height: self.height,
			xdim: self.narrow_bar_width,
			foreground: BarcodeSvgColor::new(self.foreground_color.clone()),
			background: BarcodeSvgColor::new(self.background_color.clone()),
		};
		
		use self::BarcodeVariant::*;
		
		let svgString = match self.variant
		{
			Codabar => barcodeSvg.generate(sym::codabar::Codabar::new(&self.data)?.encode())?,
			Code11 => barcodeSvg.generate(sym::code11::Code11::new(&self.data)?.encode())?,
			Code39 => barcodeSvg.generate(sym::code39::Code39::new(&self.data)?.encode())?,
			Code39Checksummed => barcodeSvg.generate(sym::code39::Code39::with_checksum(&self.data)?.encode())?,
			Code93 => barcodeSvg.generate(sym::code93::Code93::new(&self.data)?.encode())?,
			Code128 => barcodeSvg.generate(sym::code128::Code128::new(&self.data)?.encode())?,
			EAN2 => barcodeSvg.generate(sym::ean_supp::EANSUPP::EAN2(self.data.clone().into_bytes()).encode())?,
			EAN5 => barcodeSvg.generate(sym::ean_supp::EANSUPP::EAN5(self.data.clone().into_bytes()).encode())?,
			EAN8 => barcodeSvg.generate(sym::ean8::EAN8::new(&self.data)?.encode())?,
			EAN13 => barcodeSvg.generate(sym::ean13::EAN13::new(&self.data)?.encode())?,
			TwoOfFiveInterleaved => barcodeSvg.generate(sym::tf::TF::interleaved(&self.data)?.encode())?,
			TwoOfFiveStandard => barcodeSvg.generate(sym::tf::TF::standard(&self.data)?.encode())?,
		};
		Ok(svgString)
	}
	
	#[inline(always)]
	fn foreground_color_default() -> [u8; 4]
	{
		[0x00, 0x00, 0x00, 0xFF]
	}
	
	#[inline(always)]
	fn background_color_default() -> [u8; 4]
	{
		[0xFF, 0xFF, 0xFF, 0xFF]
	}
}
