// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct QrCodeData
{
	#[serde(deserialize_with = "QrCodeData::data_deserialize")] data: Vec<u8>,
	#[serde(default)] version: QrVersion,
	#[serde(default)] error_correction_level: QrErrorCorrectionLevel,
	#[serde(default)] quiet_zone: bool, // The quiet zone is a white border
	#[serde(default = "QrCodeData::light_color_default")] light_color: String,
	#[serde(default = "QrCodeData::dark_color_default")] dark_color: String,
	#[serde(default = "QrCodeData::module_width_default")] module_width: u32,
	#[serde(default = "QrCodeData::module_height_default")] module_height: u32,
}

impl QrCodeData
{
	#[inline(always)]
	pub(crate) fn svgString(&self) -> Result<String, CordialError>
	{
		let qrCode = QrCode::with_version(&self.data, self.version.toVersion(), self.error_correction_level.toEcLevel())?;
		
		let svgString = qrCode.render::<SvgColor>()
		.light_color(SvgColor(&self.light_color))
		.dark_color(SvgColor(&self.dark_color))
		.quiet_zone(false)
		.module_dimensions(self.module_width, self.module_height)
		.build();
		
		Ok(svgString)
	}
	
	#[inline(always)]
	fn data_deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error>
	{
		struct DataVisitor;
		
		impl<'de> Visitor<'de> for DataVisitor
		{
			type Value = Vec<u8>;
			
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("a string or bytes")
			}
			
			fn visit_str<E: DeserializeError>(self, v: &str) -> Result<Self::Value, E>
			{
				Ok(v.to_owned().into_bytes())
			}
			
			fn visit_borrowed_str<E: DeserializeError>(self, v: &'de str) -> Result<Self::Value, E>
			{
				Ok(v.to_owned().into_bytes())
			}
			
			fn visit_string<E: DeserializeError>(self, v: String) -> Result<Self::Value, E>
			{
				Ok(v.into_bytes())
			}
			
			fn visit_bytes<E: DeserializeError>(self, v: &[u8]) -> Result<Self::Value, E>
			{
				Ok(v.to_vec())
			}
			
			fn visit_borrowed_bytes<E: DeserializeError>(self, v: &'de [u8]) -> Result<Self::Value, E>
			{
				Ok(v.to_vec())
			}
			
			fn visit_byte_buf<E: DeserializeError>(self, v: Vec<u8>) -> Result<Self::Value, E>
			{
				Ok(v)
			}
		}
		
		deserializer.deserialize_string(DataVisitor)
	}
	
	#[inline(always)]
	fn light_color_default() -> String
	{
		"#fff".to_owned()
	}
	
	#[inline(always)]
	fn dark_color_default() -> String
	{
		"#000".to_owned()
	}
	
	#[inline(always)]
	fn module_width_default() -> u32
	{
		8
	}
	
	#[inline(always)]
	fn module_height_default() -> u32
	{
		8
	}
}
