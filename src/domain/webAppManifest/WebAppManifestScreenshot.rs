// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct WebAppManifestScreenshot
{
	screenshot: ResourceReference,
	platform: Option<WebAppManifestPlatform>,
}

impl Serialize for WebAppManifestScreenshot
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let mut fieldCount = 3;
		
		if self.platform.is_some()
		{
			fieldCount += 1;
		}
		
		let mut state = serializer.serialize_struct("WebAppManifestIcon", fieldCount)?;
		{
			let screenshotUrlData = WebAppManifestSerializationState::urlData::<S>(&self.screenshot)?;
			screenshotUrlData.validateIsSuitableForWebAppManifestScreenshot().map_err(|cordialError| S::Error::custom(cordialError))?;
			
			state.serialize_field("src", screenshotUrlData.url().as_str())?;
			
			state.serialize_field("type", screenshotUrlData.mimeType().as_ref())?;
			
			let (width, height) = screenshotUrlData.dimensions().map_err(|cordialError| S::Error::custom(cordialError))?;
			state.serialize_field("sizes", &format!("{}x{}", width, height))?;
			
			if let Some(ref platform) = self.platform
			{
				state.serialize_field("platform", platform)?;
			}
		}
		state.end()
	}
}
