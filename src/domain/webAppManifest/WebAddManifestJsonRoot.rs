// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct WebAddManifestJsonRoot<'a>
{
	pub(crate) ourUrlToMinifyAgainst: &'a Url,
	pub(crate) lang: Iso639Dash1Alpha2Language,
	pub(crate) webAppManifestPipeline: &'a WebAppManifestPipeline,
}

impl<'a> Serialize for WebAddManifestJsonRoot<'a>
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let webAppManifestAbstract = match self.webAppManifestPipeline.abstracts.get(&self.lang)
		{
			None => return Err(S::Error::custom(format!("No web app manifest abstract for language {:?}", self.lang))),
			Some(abstract_) => abstract_,
		};
		
		let mut state = serializer.serialize_struct("WebAddManifestJsonRoot", 7)?;
		{
			state.serialize_field("lang", &self.lang)?;
			state.serialize_field("short_name", &webAppManifestAbstract.short_name)?;
			state.serialize_field("name", &webAppManifestAbstract.name)?;
			state.serialize_field("orientation", &self.webAppManifestPipeline.orientation)?;
			state.serialize_field("prefer_related_applications", &self.webAppManifestPipeline.prefer_related_applications)?;
			state.serialize_field("icons", &self.webAppManifestPipeline.icons)?;
			
			let urlData = WebAddManifestSerializationState::urlData::<S>(&self.webAppManifestPipeline.start_url, ResourceTag::default)?;
			state.serialize_field("start_url", urlData.url_str())?;
		}
		state.end()
	}
}

impl<'a> WebAddManifestJsonRoot<'a>
{
	#[inline(never)]
	pub(crate) fn to_json_bytes(&self, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<Vec<u8>, CordialError>
	{
		WebAddManifestSerializationState::with(resources, fallbackIso639Dash1Alpha2Language, self.lang, ||
		{
			::serde_json::to_vec(self).map_err(|serializeError| CordialError::CouldNotSerializeJson(serializeError))
		}).map(|(vec, _)| vec)
	}
}
