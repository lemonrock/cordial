// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ImageMetaData
{
	#[serde(default)] pub(crate) abstracts: HashMap<Iso639Dash1Alpha2Language, Rc<ImageAbstract>>,
	
	#[serde(default)] pub(crate) license_url: ResourceReference,
	
	#[serde(default)] pub(crate) credit: FullName,
	
	/*
		A list of one or more strings separated by commas indicating a set of source sizes. Each source size consists of:
   			* a media condition. This must be omitted for the last item.
    		* a source size value.
    	eg "(min-width: 36em) 33.3vw, 100vw"  from  https://ericportis.com/posts/2014/srcset-sizes/
    	Used in either the img attribute sizes or the picture attribute sizes
	*/
	#[serde(default)] pub(crate) sizes: Option<(Vec<MediaQueryAndLength>, Length)>,

	#[serde(default)] pub(crate) id: Option<String>,
	#[serde(default)] pub(crate) classes: Vec<String>,
	
	#[serde(default)] pub(crate) is_server_side_map: bool,
	#[serde(default)] pub(crate) map_id: Option<String>, // Without leading '#'
	#[serde(default)] pub(crate) use_cross_origin_credentials: bool,
	#[serde(default)] pub(crate) referrer_policy: ReferrerPolicy,
	#[serde(default)] pub(crate) long_description: Option<LongDescription>,
	
	#[serde(default)] pub(crate) facebook_open_graph_user_generated: bool,
}

impl Default for ImageMetaData
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			abstracts: Default::default(),
			license_url: Default::default(),
			credit: Default::default(),
			sizes: None,
			id: None,
			classes: Default::default(),
			is_server_side_map: false,
			map_id: None,
			use_cross_origin_credentials: false,
			referrer_policy: Default::default(),
			long_description: None,
			facebook_open_graph_user_generated: false,
		}
	}
}

impl ImageMetaData
{
	#[inline(always)]
	pub(crate) fn imageAbstract(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&Rc<ImageAbstract>, CordialError>
	{
		match self.abstracts.get(&iso639Dash1Alpha2Language)
		{
			None => Err(CordialError::Configuration(format!("Could not find image abstract for language {}", iso639Dash1Alpha2Language))),
			Some(imageAbstract) => Ok(imageAbstract),
		}
	}
	
	#[inline(always)]
	pub(crate) fn imageAbstractWithFallback(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&Rc<ImageAbstract>, CordialError>
	{
		match self.abstracts.get(&iso639Dash1Alpha2Language)
		{
			None => match self.abstracts.get(&fallbackIso639Dash1Alpha2Language)
			{
				None => Err(CordialError::Configuration(format!("Could not find image abstract for '{:?}' or fallback '{:?}'", iso639Dash1Alpha2Language, fallbackIso639Dash1Alpha2Language))),
				Some(primaryImageAbstract) => Ok(primaryImageAbstract)
			},
			Some(imageAbstract) => Ok(imageAbstract),
		}
	}
	
	#[inline(always)]
	pub(crate) fn alt(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(&Rc<String>), CordialError>
	{
		Ok(&self.imageAbstractWithFallback(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?.alt)
	}
	
	#[inline(always)]
	pub(crate) fn licenseUrlAndAnchorTitleAttribute<'a>(&self, resources: &'a Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(Rc<Url>, Rc<String>), CordialError>
	{
		self.license_url.urlAndAnchorTitleAttribute(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn addToImgAttributes(&self, imgAttributes: &mut Vec<Attribute>, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, isForAmp: bool) -> Result<(), CordialError>
	{
		if let Some(ref sizes) = self.sizes
		{
			let mut sizesAttribute = String::new();
			for size in sizes.0.iter()
			{
				sizesAttribute.push_str(&size.media);
				sizesAttribute.push(' ');
				sizesAttribute.push_str(&size.length);
				sizesAttribute.push(',');
			}
			sizesAttribute.push_str(&sizes.1);
			
			imgAttributes.push("sizes".string_attribute(sizesAttribute));
		}
		
		if let Some(ref id) = self.id
		{
			imgAttributes.push("id".str_attribute(id));
		}
		
		if self.classes.len() > 0
		{
			imgAttributes.push("class".space_separated_attribute(self.classes.iter().map(|string| string.as_ref())));
		}
		
		if !isForAmp
		{
			if self.is_server_side_map
			{
				imgAttributes.push("ismap".empty_attribute());
			}
			
			if let Some(ref map_id) = self.map_id
			{
				imgAttributes.push("map".string_attribute(format!("#{}", map_id)));
			}
			
			if self.use_cross_origin_credentials
			{
				imgAttributes.push("crossorigin".str_attribute("use-credentials"));
			}
			
			self.referrer_policy.addToImgAttributes(imgAttributes);
			
			if let Some(ref longDescription) = self.long_description
			{
				longDescription.addToImgAttributes(imgAttributes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn anchorTitleAttribute(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<Option<Rc<String>>, CordialError>
	{
		let imageAbstract = self.imageAbstractWithFallback(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		Ok(Some(imageAbstract.title.clone()))
	}
	
	#[inline(always)]
	pub(crate) fn rssImage(&self, internalImage: &ResourceReference, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, resources: &Resources) -> Result<RssImage, CordialError>
	{
		let imageAbstract = (*self.imageAbstractWithFallback(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?).clone();
		let urlData = internalImage.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
		
		let (width, height, fileSize) = urlData.image()?;
		
		Ok
		(
			RssImage
			{
				width,
				height,
				url: urlData.url().clone(),
				fileSize,
				mimeType: urlData.mimeType().clone(),
				imageAbstract,
				credit: self.credit.clone(),
				iso639Dash1Alpha2Language,
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn siteMapWebPageImage(&self, internalImage: &ResourceReference, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, resources: &Resources) -> Result<SiteMapWebPageImage, CordialError>
	{
		let imageAbstract = self.imageAbstractWithFallback(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		let iso639Dash1Alpha2Language = Some(iso639Dash1Alpha2Language);
		
		let url = internalImage.urlMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?.clone();
		let licenseUrl = self.license_url.urlMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?.clone();
		let imageAbstract = imageAbstract.clone();
		
		Ok(SiteMapWebPageImage { url, licenseUrl, imageAbstract })
	}
}
