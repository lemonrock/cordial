// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct WebAppManifestIcon
{
	icon_url: ResourceUrl,
	size: u32,
	#[serde(default)] density: WebAppManifestIconPixelDensity,
}

impl Serialize for WebAppManifestIcon
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let (serializeDensity, fieldCount) = if self.density.isDefault()
		{
			(false, 3)
		}
		else
		{
			(true, 4)
		};
		
		let mut state = serializer.serialize_struct("WebAppManifestIcon", fieldCount)?;
		{
			let iconUrlData = WebAddManifestSerializationState::urlData::<S>(&self.icon_url, ResourceTag::default)?;
			iconUrlData.validateIsPng().map_err(|cordialError| S::Error::custom(cordialError))?;
			
			state.serialize_field("src", iconUrlData.url().as_str())?;
			
			state.serialize_field("type", iconUrlData.mimeType().as_ref())?;
			
			let (width, height) = iconUrlData.dimensions().map_err(|cordialError| S::Error::custom(cordialError))?;
			state.serialize_field("sizes", &format!("{}x{}", width, height))?;
			
			if serializeDensity
			{
				state.serialize_field("density", &self.density)?;
			}
			else
			{
				state.skip_field("density")?;
			}
		}
		state.end()
	}
}
