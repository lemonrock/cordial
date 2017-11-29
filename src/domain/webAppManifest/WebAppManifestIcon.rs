// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct WebAppManifestIcon
{
	icon: ResourceReference,
	#[serde(default)] density: WebAppManifestIconPixelDensity,
	purposes: BTreeSet<WebAppManifestIconPurpose>,
	platform: Option<WebAppManifestPlatform>,
}

impl Serialize for WebAppManifestIcon
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let mut fieldCount = 3;
		
		let serializeDensity = if self.density.isDefault()
		{
			fieldCount += 1;
			false
		}
		else
		{
			true
		};
		
		if !self.purposes.is_empty()
		{
			fieldCount += 1;
		}
		
		if self.platform.is_some()
		{
			fieldCount += 1;
		}
		
		let mut state = serializer.serialize_struct("WebAppManifestIcon", fieldCount)?;
		{
			let iconUrlData = WebAppManifestSerializationState::urlData::<S>(&self.icon)?;
			iconUrlData.validateIsSuitableForWebAppManifestIcon().map_err(|cordialError| S::Error::custom(cordialError))?;
			
			state.serialize_field("src", iconUrlData.url().as_str())?;
			
			state.serialize_field("type", iconUrlData.mimeType().as_ref())?;
			
			let (width, height) = iconUrlData.dimensions().map_err(|cordialError| S::Error::custom(cordialError))?;
			if width != height
			{
				return Err(S::Error::custom("width and height must be square for a web app manifest icon"));
			}
			state.serialize_field("sizes", &format!("{}x{}", width, height))?;
			
			if serializeDensity
			{
				state.serialize_field("density", &self.density)?;
			}
			
			if !self.purposes.is_empty()
			{
				let mut concatenated = String::new();
				let mut afterFirst = false;
				for purpose in self.purposes.iter()
				{
					if afterFirst
					{
						concatenated.push(' ');
					}
					else
					{
						afterFirst = true;
					}
					concatenated.push_str(purpose.to_str())
				}
				state.serialize_field("purpose", &concatenated)?;
			}
			
			if let Some(ref platform) = self.platform
			{
				state.serialize_field("platform", platform)?;
			}
		}
		state.end()
	}
}
