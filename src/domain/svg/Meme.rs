// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Meme
{
	#[serde(default = "Meme::text_width_default")] text_width: f32,
	#[serde(default = "Meme::text_height_default")] text_height: f32,
}

impl Meme
{
	#[inline(always)]
	pub(crate) fn svgString(&self, inputContentFilePath: &Path) -> Result<String, CordialError>
	{
		use ::memenhancer::to_svg;
		
		let content = inputContentFilePath.fileContentsAsString().context(inputContentFilePath)?;
		
		let svgDocument = to_svg(&content, self.text_width, self.text_height);
		Ok(format!("{}", svgDocument))
	}
	
	#[inline(always)]
	fn text_width_default() -> f32
	{
		8.0
	}
	
	#[inline(always)]
	fn text_height_default() -> f32
	{
		16.0
	}
}
