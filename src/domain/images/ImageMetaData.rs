// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ImageMetaData
{
	pub(crate) abstracts: HashMap<String, ImageAbstract>,
	
	pub(crate) license_url: ResourceReference,
	
	pub(crate) credit: FullName,
	
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
}

impl ImageMetaData
{
	#[inline(always)]
	pub(crate) fn abstract_(&self, iso_639_1_alpha_2_language_code: &str) -> Result<&ImageAbstract, CordialError>
	{
		match self.abstracts.get(iso_639_1_alpha_2_language_code)
		{
			None => Err(CordialError::Configuration(format!("Could not find abstract for language {}", iso_639_1_alpha_2_language_code))),
			Some(abstract_) => Ok(abstract_),
		}
	}
	
	pub(crate) fn sizesAttribute(&self) -> Option<String>
	{
		if let Some(ref sizes) = self.sizes
		{
			let mut buffer = String::new();
			for size in sizes.0.iter()
			{
				buffer.push_str(&size.media);
				buffer.push(' ');
				buffer.push_str(&size.length);
				buffer.push(',');
			}
			buffer.push_str(&sizes.1);
			
			Some(buffer)
		}
		else
		{
			None
		}
	}
	
	pub(crate) fn siteMapWebPageImage(&self, internalImage: &ResourceReference, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: &str, resources: &Resources) -> Result<SiteMapWebPageImage, CordialError>
	{
		let url = match resources.urlData(internalImage, primary_iso_639_1_alpha_2_language_code, Some(iso_639_1_alpha_2_language_code))?
		{
			None => return Err(CordialError::Configuration(format!("Could not locate a resource for url '{:?}'", internalImage))),
			Some(urlData) => urlData.urlOrDataUri.deref().clone(),
		};
		
		let licenseUrl = match resources.urlData(&self.license_url, primary_iso_639_1_alpha_2_language_code, None)?
		{
			None => return Err(CordialError::Configuration(format!("Could not locate a resource for license url '{:?}'", &self.license_url))),
			Some(urlData) => urlData.urlOrDataUri.deref().clone(),
		};
		
		match self.abstracts.get(primary_iso_639_1_alpha_2_language_code)
		{
			None => Err(CordialError::Configuration(format!("Could not locate an image abstract for '{:?}'", primary_iso_639_1_alpha_2_language_code))),
			Some(ref abstract_) => Ok(SiteMapWebPageImage
			{
				url,
				caption: abstract_.caption.clone(),
				geographicLocation: abstract_.geographic_location.clone(),
				title: abstract_.title.clone(),
				licenseUrl,
			})
		}
	}
	
	// TODO: add <img> with a class of webfeedsFeaturedVisual for feedly OR if first img > 450px OR feedly will try to poll website for open graph or twitter card
	pub(crate) fn rssImage(&self, internalImage: &ResourceReference, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: &str, resources: &Resources) -> Result<RssImage, CordialError>
	{
		let alt = match self.abstracts.get(iso_639_1_alpha_2_language_code)
		{
			None => match self.abstracts.get(primary_iso_639_1_alpha_2_language_code)
			{
				None => return Err(CordialError::Configuration(format!("Could not locate an image abstract for '{:?}'", primary_iso_639_1_alpha_2_language_code))),
				Some(ref primaryAbstract) => &primaryAbstract.alt
			},
			Some(ref abstract_) => &abstract_.alt
		};
		
		match resources.urlData(internalImage, primary_iso_639_1_alpha_2_language_code, Some(iso_639_1_alpha_2_language_code))?
		{
			None => Err(CordialError::Configuration(format!("Could not find article image for RSS feed for '{:?}'", internalImage))),
			
			Some(urlData) =>
			{
				let jsonValue = &urlData.jsonValue;
				
				Ok
				(
					RssImage
					{
						width: jsonValue.u32("width")?,
						height: jsonValue.u32("height")?,
						url: urlData.urlOrDataUri.deref().clone(),
						fileSize: jsonValue.u64("size")?,
						mimeType: jsonValue.mime("mime")?,
						alt: alt.clone(),
						credit: self.credit.clone(),
						iso_639_1_alpha_2_language_code: iso_639_1_alpha_2_language_code.to_owned(),
					}
				)
			}
		}
	}
}