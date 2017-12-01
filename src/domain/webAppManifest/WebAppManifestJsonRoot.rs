// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct WebAppManifestJsonRoot<'a>
{
	pub(crate) ourUrlToMinifyAgainst: &'a Url,
	pub(crate) languageData: &'a LanguageData<'a>,
	pub(crate) webAppManifestPipeline: &'a WebAppManifestPipeline,
}

impl<'a> Serialize for WebAppManifestJsonRoot<'a>
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let webAppManifestAbstract = match self.webAppManifestPipeline.abstracts.get(&self.iso639Dash1Alpha2Language())
		{
			None => return Err(S::Error::custom(format!("No web app manifest abstract for language {:?}", self.iso639Dash1Alpha2Language()))),
			Some(webAppManifestAbstract) => webAppManifestAbstract,
		};
		
		let mut fieldCount = 13;
		
		if self.webAppManifestPipeline.iarc_rating_id.is_some()
		{
			fieldCount += 1;
		}
		
		if self.webAppManifestPipeline.iarc_rating_id.is_some()
		{
			fieldCount += 1;
		}
		
		if self.webAppManifestPipeline.orientation.is_some()
		{
			fieldCount += 1;
		}
		
		if self.webAppManifestPipeline.background_css_color.is_some()
		{
			fieldCount += 1;
		}
		
		if self.webAppManifestPipeline.service_worker.is_some()
		{
			fieldCount += 1;
		}
		
		let mut state = serializer.serialize_struct("WebAppManifestJsonRoot", fieldCount)?;
		{
			state.serialize_field("dir", match self.languageData.dir()
			{
				Dir::LeftToRight => "ltr",
				Dir::RightToLeft => "rtl",
				Dir::Automatic => "auto",
			})?; // defaults to auto
			state.serialize_field("lang", &self.iso639Dash1Alpha2Language())?;
			state.serialize_field("name", &webAppManifestAbstract.name)?;
			state.serialize_field("short_name", &webAppManifestAbstract.short_name)?;
			state.serialize_field("description", &webAppManifestAbstract.description)?;
			state.serialize_field("icons", &self.webAppManifestPipeline.icons)?;
			state.serialize_field("screenshots", &self.webAppManifestPipeline.screenshots)?;
			state.serialize_field("categories", &self.webAppManifestPipeline.categories)?;
			if let Some(ref iarc_rating_id) = self.webAppManifestPipeline.iarc_rating_id
			{
				state.serialize_field("iarc_rating_id", iarc_rating_id.as_str())?;
			}
			let startUrlData = WebAppManifestSerializationState::urlDataFrom::<S>(&self.webAppManifestPipeline.start_url, ResourceTag::default)?;
			startUrlData.validateIsHtml().map_err(|cordialError| S::Error::custom(cordialError))?;
			state.serialize_field("start_url", startUrlData.url_str())?;
			state.serialize_field("display", &self.webAppManifestPipeline.display)?; // defaults to browser
			if let Some(ref orientation) = self.webAppManifestPipeline.orientation
			{
				state.serialize_field("orientation", orientation)?;
			}
			if let Some(ref themeCssColor) = self.webAppManifestPipeline.theme_css_color
			{
				state.serialize_field("theme_color", themeCssColor.as_str())?;
			}
			if let Some(ref backgroundCssColor) = self.webAppManifestPipeline.background_css_color
			{
				state.serialize_field("background_color", backgroundCssColor.as_str())?;
			}
			let scopeUrlData = WebAppManifestSerializationState::urlDataFrom::<S>(&self.webAppManifestPipeline.scope, ResourceTag::default)?;
			scopeUrlData.validateIsHtml().map_err(|cordialError| S::Error::custom(cordialError))?;
			state.serialize_field("scope", scopeUrlData.url_str())?;
			if let Some(ref serviceWorker) = self.webAppManifestPipeline.service_worker
			{
				state.serialize_field("serviceworker", serviceWorker)?;
			}
			state.serialize_field("related_applications", &self.webAppManifestPipeline.related_applications)?;
			state.serialize_field("prefer_related_applications", &self.webAppManifestPipeline.prefer_related_applications)?;// defaults to false
		}
		state.end()
	}
}

impl<'a> WebAppManifestJsonRoot<'a>
{
	#[inline(never)]
	pub(crate) fn iso639Dash1Alpha2Language(&self) -> Iso639Dash1Alpha2Language
	{
		self.languageData.iso639Dash1Alpha2Language
	}
	
	#[inline(never)]
	pub(crate) fn to_json_bytes(&self, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<Vec<u8>, CordialError>
	{
		WebAppManifestSerializationState::with(resources, fallbackIso639Dash1Alpha2Language, self.iso639Dash1Alpha2Language(), ||
		{
			::serde_json::to_vec(self).map_err(|serializeError| CordialError::CouldNotSerializeJson(serializeError))
		}).map(|(vec, _)| vec)
	}
}
