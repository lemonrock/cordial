// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SiteMap
{
	#[serde(default)] headers: HashMap<String, String>,
	#[serde(default = "SiteMap::max_age_in_seconds_default")] max_age_in_seconds: u32,
	#[serde(default)] compression: compression,
}

impl Default for SiteMap
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			headers: HashMap::default(),
			max_age_in_seconds: 60,
			compression: compression::default(),
		}
	}
}

impl SiteMap
{
	#[inline(always)]
	pub(crate) fn renderResource<'a>(&'a self, languageData: (&str, &language), handlebars: &mut Handlebars, configuration: &Configuration, resources: &mut Resources, oldResources: &Arc<Resources>, siteMapIndexUrls: &mut BTreeSet<Url>, webPages: &[SiteMapWebPage]) -> Result<(), CordialError>
	{
		let primary_iso_639_1_alpha_2_language_code = languageData.0;
		let siteMapBaseUrlWithTrailingSlash = languageData.1.baseUrl()?;
		
		let namespace = Namespace
		(
			btreemap!
			{
				NS_NO_PREFIX.to_owned() => "http://www.sitemaps.org/schemas/sitemap/0.9".to_owned(),
			}
		);
		
		let emptyAttributes = [];
		
		let mut siteMaps = self.writeSiteMapFiles(languageData, handlebars, configuration, webPages)?;
		
		let mut siteMaps = siteMaps.drain(..);
		let mut keepLooping = true;
		let mut index = 0;
		while keepLooping
		{
			const MaximumNumberOfUrlsInASiteMapIndex: usize = 50_000;
			const MaximumSiteMapFileSizeInBytes: usize = 52_428_800;
			const SafeMaximumSiteMapIndexFileSizeInBytes: usize = MaximumSiteMapFileSizeInBytes - 1024;
			
			let bytesWritten = Cell::new(0);
			let mut eventWriter = Self::createEventWriter(&bytesWritten);
			
			eventWriter.writeBasicXmlDocumentPreamble()?;
			
			let mut count = 0;
			eventWriter.writeWithinElement(Name::local("sitemapindex"), &namespace, &emptyAttributes, |eventWriter|
			{
				while count <= MaximumNumberOfUrlsInASiteMapIndex && bytesWritten.get() < SafeMaximumSiteMapIndexFileSizeInBytes
				{
					match siteMaps.next()
					{
						None =>
						{
							keepLooping = false;
							return Ok(());
						}
						Some((url, currentResponse)) =>
						{
							let namespace = &namespace;
							let emptyAttributes = &emptyAttributes;
							let resources = &mut *resources;
							eventWriter.writeWithinElement(Name::local("sitemap"), namespace, emptyAttributes, move |eventWriter|
							{
								eventWriter.writeUnprefixedTextElement(namespace, emptyAttributes, "loc", url.as_ref())?;
								
								let lastModifiedHttpDate = resources.addResource(url, currentResponse, oldResources.clone());
								let lastModifiedTimeStamp: DateTime<Utc> = DateTime::from(SystemTime::from(lastModifiedHttpDate));
								
								eventWriter.writeUnprefixedTextElement(namespace, emptyAttributes, "lastmod", &lastModifiedTimeStamp.to_rfc3339())
							})?;
							count += 1;
						}
					}
				}
				Ok(())
			})?;
			
			let unversionedCanonicalUrl = siteMapBaseUrlWithTrailingSlash.join(&format!("sitemap-index.{}.{}.xmlExtra", index, primary_iso_639_1_alpha_2_language_code)).unwrap();
			let headers = generateHeaders(handlebars, &self.headers, Some(languageData), HtmlVariant::Canonical, configuration, true, self.max_age_in_seconds, true, &unversionedCanonicalUrl)?;
			let siteMapIndexBodyUncompressed = eventWriter.into_inner().bytes();
			let siteMapIndexBodyCompressed = self.compression.compress(&siteMapIndexBodyUncompressed)?;
			
			let xmlMimeType = "application/xmlExtra; charset=utf-8".parse().unwrap();
			let staticResponse = StaticResponse::new(StatusCode::Ok, ContentType(xmlMimeType), headers, siteMapIndexBodyUncompressed, Some(siteMapIndexBodyCompressed));
			
			siteMapIndexUrls.insert(unversionedCanonicalUrl.clone());
			resources.addResource(unversionedCanonicalUrl, RegularAndPjaxStaticResponse::regular(staticResponse), oldResources.clone());
			
			index += 1;
		}
		
		Ok(())
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn writeSiteMapFiles<'a>(&'a self, languageData: (&str, &language), handlebars: &mut Handlebars, configuration: &Configuration, webPages: &[SiteMapWebPage]) -> Result<Vec<(Url, RegularAndPjaxStaticResponse)>, CordialError>
	{
		let primary_iso_639_1_alpha_2_language_code = languageData.0;
		let siteMapBaseUrlWithTrailingSlash = languageData.1.baseUrl()?;
		
		let namespace = Namespace
		(
			btreemap!
			{
				NS_NO_PREFIX.to_owned() => "http://www.sitemaps.org/schemas/sitemap/0.9".to_owned(),
				"xhtml".to_owned() => "http://www.w3.org/1999/xhtml".to_owned(),
				"image".to_owned() => "http://www.google.com/schemas/sitemap-image/1.1".to_owned(),
				"video".to_owned() => "http://www.google.com/schemas/sitemap-video/1.1".to_owned(),
			}
		);
		
		let emptyAttributes = [];
		
		let mut urlAndResponse = Vec::with_capacity(1);
		let mut startingIndex = 0;
		while startingIndex < webPages.len()
		{
			const MaximumNumberOfUrlsInASiteMap: usize = 50_000;
			const MaximumSiteMapFileSizeInBytes: usize = 52_428_800;
			const SafeMaximumSiteMapFileSizeInBytes: usize = MaximumSiteMapFileSizeInBytes - 1024;
			
			let bytesWritten = Cell::new(0);
			let mut eventWriter = Self::createEventWriter(&bytesWritten);
			
			let webPagesForThisSiteMapFile = &webPages[startingIndex .. ];
			let mut count = 0;
			
			eventWriter.writeBasicXmlDocumentPreamble()?;
			
			eventWriter.writeWithinElement(Name::local("urlset"), &namespace, &emptyAttributes, |eventWriter|
			{
				for webPage in webPagesForThisSiteMapFile.iter()
				{
					startingIndex += 1;
					
					if webPage.writeXml(primary_iso_639_1_alpha_2_language_code, eventWriter, &namespace, &emptyAttributes)?
					{
						count += 1;
						
						if count == MaximumNumberOfUrlsInASiteMap
						{
							return Ok(())
						}
						
						if bytesWritten.get() >= SafeMaximumSiteMapFileSizeInBytes
						{
							return Ok(())
						}
					}
				}
				
				Ok(())
			})?;
			
			let unversionedCanonicalUrl = siteMapBaseUrlWithTrailingSlash.join(&format!("sitemap.{}.{}.xmlExtra", urlAndResponse.len(), primary_iso_639_1_alpha_2_language_code)).unwrap();
			let headers = generateHeaders(handlebars, &self.headers, Some(languageData), HtmlVariant::Canonical, configuration, true, self.max_age_in_seconds, true, &unversionedCanonicalUrl)?;
			let siteMapBodyUncompressed = eventWriter.into_inner().bytes();
			let siteMapBodyCompressed = self.compression.compress(&siteMapBodyUncompressed)?;
			
			let xmlMimeType = "application/xmlExtra; charset=utf-8".parse().unwrap();
			let staticResponse = StaticResponse::new(StatusCode::Ok, ContentType(xmlMimeType), headers, siteMapBodyUncompressed, Some(siteMapBodyCompressed));
			
			urlAndResponse.push((unversionedCanonicalUrl, RegularAndPjaxStaticResponse::regular(staticResponse)));
		}
		
		Ok(urlAndResponse)
	}
	
	#[inline(always)]
	fn createEventWriter<'a>(bytesWritten: &'a Cell<usize>) -> EventWriter<LengthTrackingWriter<'a>>
	{
		let configuration = EmitterConfig
		{
			line_separator: Cow::Borrowed(""),
			indent_string: Cow::Borrowed(""),
			perform_indent: false,
			perform_escaping: true,
			write_document_declaration: true,
			normalize_empty_elements: true,
			cdata_to_characters: true,
			keep_element_names_stack: true,
			autopad_comments: false,
		};
		configuration.create_writer(LengthTrackingWriter::new(bytesWritten))
	}
	
	#[inline(always)]
	fn max_age_in_seconds_default() -> u32
	{
		60
	}
}
