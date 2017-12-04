// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct FacebookOpenGraph
{
	#[serde(default)] pub(crate) site_name: Option<String>,
	#[serde(default)] pub(crate) admins: Vec<FacebookId>,
	#[serde(default)] pub(crate) app_id: Option<String>,
	#[serde(default)] pub(crate) pages: Vec<FacebookId>,
	#[serde(default)] pub(crate) profile_id: Option<FacebookId>,
	#[serde(default)] pub(crate) determiner: FacebookOpenGraphDeterminer,
	#[serde(default)] pub(crate) country_restrictions: HashMap<Iso3166Dash1Alpha2CountryCode, CountryRestrictionInclusion>,
	#[serde(default)] pub(crate) age_restriction: Option<FacebookOpenGraphAgeRestriction>,
	#[serde(default)] pub(crate) content_restrictions: HashSet<FacebookOpenGraphContentRestriction>,
	#[serde(default = "FacebookOpenGraph::is_rich_attachment_default")] pub(crate) is_rich_attachment: bool,
	#[serde(default)] pub(crate) see_also: HashSet<ResourceUrl>,
	#[serde(default = "FacebookOpenGraph::ttl_in_seconds_default")] pub(crate) ttl_in_seconds: u64,
	#[serde(rename = "type")] pub(crate) type_: FacebookOpenGraphType,
}

impl Default for FacebookOpenGraph
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			site_name: None,
			admins: Default::default(),
			app_id: None,
			pages: Default::default(),
			profile_id: None,
			determiner: Default::default(),
			country_restrictions: Default::default(),
			age_restriction: None,
			content_restrictions: Default::default(),
			is_rich_attachment: Self::is_rich_attachment_default(),
			see_also: Default::default(),
			ttl_in_seconds: Self::ttl_in_seconds_default(),
			type_: Default::default(),
		}
	}
}

impl FacebookOpenGraph
{
	const SecondsPerMinute: u64 = 60;
	const MinutesPerHour: u64 = 60;
	const HoursPerDay: u64 = 24;
	const SecondsPerDay: u64 = Self::SecondsPerMinute * Self::MinutesPerHour * Self::HoursPerDay;
	const FacebookDefaultOfSevenDays: u64 = 7 * Self::SecondsPerDay;
	const FacebookMinimumOfFourDays: u64 = 4 * Self::SecondsPerDay;
	
	#[inline(always)]
	pub(crate) fn hasFacebookOpenGraphTypeDiscriminant(&self, facebookOpenGraphTypeDiscriminant: FacebookOpenGraphTypeDiscriminant) -> bool
	{
		self.type_.hasFacebookOpenGraphTypeDiscriminant(facebookOpenGraphTypeDiscriminant)
	}
	
	#[inline(always)]
	pub fn facebookOpenGraph(&self, endHeadNodes: &mut Vec<UnattachedNode>, title: &str, description: &str, canonicalUrl: &Url, publicationDate: Option<DateTime<Utc>>, lastModificationDateOrPublicationDate: Option<DateTime<Utc>>, expirationDate: Option<DateTime<Utc>>, configuration: &Configuration, resources: &Resources, articleImage: &Option<(ResourceUrl, Rc<ImageMetaData>)>, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, languageData: &LanguageData) -> Result<(), CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		endHeadNodes.push(meta_with_property_and_content("og:title", title));
		endHeadNodes.push(meta_with_property_and_content("og:description", description));
		if let Some(ref site_name) = self.site_name
		{
			endHeadNodes.push(meta_with_property_and_content("og:site_name", site_name.as_str()));
		}
		endHeadNodes.push(meta_with_property_and_content("og:url", canonicalUrl.as_ref()));
		if let Some(lastModificationDateOrPublicationDate) = lastModificationDateOrPublicationDate
		{
			endHeadNodes.push(meta_with_property_and_content("og:updated_time", &lastModificationDateOrPublicationDate.to_rfc3339()));
		}
		endHeadNodes.push(meta_with_property_and_content("og:locale", configuration.facebookOpenGraphLocaleStr(iso639Dash1Alpha2Language)?));
		
		for language in configuration.otherLanguages(iso639Dash1Alpha2Language).values()
		{
			endHeadNodes.push(meta_with_property_and_content("og:locale:alternate", language.facebookOpenGraphLocaleStr()));
		}
		
		self.addTo(endHeadNodes, fallbackIso639Dash1Alpha2Language, languageData, resources, publicationDate, lastModificationDateOrPublicationDate, expirationDate)?;
		
		if let &Some((ref articleImageResourceUrl, ref articleImageMetaData)) = articleImage
		{
			let resource = articleImageResourceUrl.resourceMandatory(resources)?;
			let urlData = resource.findUrlDataForFacebookOpenGraphImage(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
			
			let (width, height) = urlData.dimensions()?;
			
			endHeadNodes.push(meta_with_property_and_content("og:image", urlData.url_str()));
			endHeadNodes.push(meta_with_property_and_content("og:image:secure_url", urlData.url_str()));
			endHeadNodes.push(meta_with_property_and_content("og:image:type", urlData.mimeType.as_ref()));
			endHeadNodes.push(meta_with_property_and_content("og:image:width", &format!("{}", width)));
			endHeadNodes.push(meta_with_property_and_content("og:image:height", &format!("{}", height)));
			endHeadNodes.push(meta_with_property_and_content("og:image:alt", articleImageMetaData.alt(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?.as_str()));
			
			if articleImageMetaData.facebook_open_graph_user_generated
			{
				endHeadNodes.push(meta_with_property_and_content("og:image:user_generated", "1"));
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, languageData: &LanguageData, resources: &Resources, publicationDate: Option<DateTime<Utc>>, lastModificationDateOrPublicationDate: Option<DateTime<Utc>>, expirationDate: Option<DateTime<Utc>>) -> Result<(), CordialError>
	{
		const SeeAlsoTag: ResourceTag = ResourceTag::default;
		
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		for admin in self.admins.iter()
		{
			endHeadNodes.push(meta_with_property_and_content("fb:admins", admin.as_str()))
		}
		
		if let Some(ref app_id) = self.app_id
		{
			endHeadNodes.push(meta_with_property_and_content("fb:app_id", app_id.as_str()))
		}
		
		for page in self.pages.iter()
		{
			endHeadNodes.push(meta_with_property_and_content("fb:pages", page.as_str()))
		}
		
		if let Some(ref profile_id) = self.profile_id
		{
			endHeadNodes.push(meta_with_property_and_content("fb:ref profile_id", profile_id.as_str()))
		}
		
		self.determiner.addTo(endHeadNodes);
		
		use self::CountryRestrictionInclusion::*;
		
		for country in self.country_restrictions.iter().filter(|&(_country, restriction)| *restriction == whitelist).map(|(country, _restriction)| country)
		{
			endHeadNodes.push(meta_with_property_and_content("og:restrictions:country:allowed", country.to_iso_3166_1_alpha_2_language_code()));
		}
		
		for country in self.country_restrictions.iter().filter(|&(_country, restriction)| *restriction == blacklist).map(|(country, _restriction)| country)
		{
			endHeadNodes.push(meta_with_property_and_content("og:restrictions:country:disallowed", country.to_iso_3166_1_alpha_2_language_code()));
		}
		
		for country_restriction in self.content_restrictions.iter()
		{
			country_restriction.addTo(endHeadNodes);
		}
		
		if let Some(age_restriction) = self.age_restriction
		{
			age_restriction.addTo(endHeadNodes);
		}
		
		for content_restriction in self.content_restrictions.iter()
		{
			content_restriction.addTo(endHeadNodes);
		}
		
		if self.is_rich_attachment
		{
			endHeadNodes.push(meta_with_property_and_content("og:rich_attachment", "1"));
		}
		
		for seeAlso in self.see_also.iter()
		{
			let seeAlso = ResourceReference
			{
				resource: seeAlso.clone(),
				tag: SeeAlsoTag,
			};
			let url = seeAlso.urlMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
			endHeadNodes.push(meta_with_property_and_content("og:see_also", url.as_str()))
		}
		
		let ttl_in_seconds = self.ttl_in_seconds;
		if ttl_in_seconds != Self::FacebookDefaultOfSevenDays
		{
			if ttl_in_seconds < Self::FacebookMinimumOfFourDays
			{
				return Err(CordialError::Configuration(format!("The minimum value for ttl_in_seconds for Facebook is {}; {} is lower", Self::FacebookMinimumOfFourDays, ttl_in_seconds)))
			}
			endHeadNodes.push(meta_with_property_and_content("og:ttl", &format!("{}", ttl_in_seconds)));
		}
		
		self.type_.addTo(endHeadNodes, resources, fallbackIso639Dash1Alpha2Language, languageData, publicationDate, lastModificationDateOrPublicationDate, expirationDate)?;
		
		Ok(())
	}
	
	#[inline(always)]
	fn is_rich_attachment_default() -> bool
	{
		true
	}
	
	#[inline(always)]
	fn ttl_in_seconds_default() -> u64
	{
		Self::FacebookDefaultOfSevenDays
	}
}
