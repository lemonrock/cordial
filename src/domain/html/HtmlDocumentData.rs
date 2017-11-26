// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct HtmlDocumentData<'a>
{
	pub(crate) fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language,
	pub(crate) iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language,
	pub(crate) markdownParser: MarkdownParser,
	pub(crate) markdown: String,
	pub(crate) markdownPluginData: MarkdownPluginData<'a>,
	pub(crate) htmlAbstract: Rc<HtmlAbstract>,
	pub(crate) articleImage: Option<(ResourceReference, Rc<ImageMetaData>)>,
	pub(crate) siteMapImages: &'a [ResourceUrl],
	pub(crate) publicationDate: Option<DateTime<Utc>>,
	pub(crate) lastModificationDateOrPublicationDate: Option<DateTime<Utc>>,
	pub(crate) modifications: BTreeMap<DateTime<Utc>, Rc<String>>,
	pub(crate) expirationDate: Option<DateTime<Utc>>,
	pub(crate) configuration: &'a Configuration,
	pub(crate) htmlUrls: HtmlUrls<'a>,
}

impl<'a> HtmlDocumentData<'a>
{
	#[inline(always)]
	pub(crate) fn addToRssChannels(&self, resources: &Resources, rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, author: &Rc<EMailAddress>, rssChannelsToCategories: &OrderMap<Rc<RssChannelName>, Rc<BTreeSet<RssCategoryName>>>, inputContentFilePath: &Path, handlebars: &HandlebarsWrapper) -> Result<(), CordialError>
	{
		for (rssChannelName, categories) in rssChannelsToCategories.iter()
		{
			match rssChannelsToRssItems.get_mut(rssChannelName)
			{
				None => return Err(CordialError::Configuration(format!("RSS channel '{}' does not have a configuration", rssChannelName))),
				Some(mut rssItems) =>
				{
					const IsNotForAmp: bool = false;
					let (_document, rssHtml) = self.renderHtmlDocument(inputContentFilePath, IsNotForAmp, handlebars, rssChannelName)?;
					rssItems.push
					(
						RssItem
						{
							rssItemLanguageVariant: RssItemLanguageVariant
							{
								webPageDescription: self.htmlAbstract.description.clone(),
								webPageUsefulContentHtml: rssHtml,
								languageSpecificUrl: self.htmlUrls.linkHeaderCanonicalUrl()?,
								primaryImage: match self.articleImage
								{
									None => None,
									Some((ref imageResourceUrl, ref articleImage)) => Some(articleImage.rssImage(imageResourceUrl, self.fallbackIso639Dash1Alpha2Language, self.iso639Dash1Alpha2Language, resources)?)
								},
							},
							lastModificationDate: self.lastModificationDateOrPublicationDate,
							author: author.clone(),
							categories: categories.clone(),
						}
					);
				}
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn addToSiteMaps(&self, resources: &Resources, siteMapWebPages: &mut Vec<SiteMapWebPage>, changeFrequency: SiteMapChangeFrequency, priority: SiteMapPriority) -> Result<(), CordialError>
	{
		let mut images = vec![];
		if let Some((ref imageResourceUrl, ref articleImage)) = self.articleImage
		{
			images.push(articleImage.siteMapWebPageImage(imageResourceUrl, self.fallbackIso639Dash1Alpha2Language, self.iso639Dash1Alpha2Language, resources)?);
		};
		
		for siteMapImageResourceUrl in self.siteMapImages.iter()
		{
			let resourceRef = siteMapImageResourceUrl.resourceMandatory(resources)?;
			let imageMetaData = resourceRef.imageMetaData()?;
			
			let internalImage = ResourceReference
			{
				resource: siteMapImageResourceUrl.clone(),
				tag: ResourceTag::largest_image,
			};
			images.push(imageMetaData.siteMapWebPageImage(&internalImage, self.fallbackIso639Dash1Alpha2Language, self.iso639Dash1Alpha2Language, resources)?)
		}
		
		siteMapWebPages.push
		(
			SiteMapWebPage
			{
				lastModified: self.lastModificationDateOrPublicationDate,
				changeFrequency,
				priority,
				urlsByIso639Dash1Alpha2Language: self.htmlUrls.linkHeaderAlternativeLanguageUrlsIncludingSelf()?,
				images
			}
		);
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn renderHtmlDocument(&self, inputContentFilePath: &Path, isForAmp: bool, handlebars: &HandlebarsWrapper, handlebarsTemplate: &str) -> Result<(RcDom, Vec<u8>), CordialError>
	{
		/*
			nodesForHtmlHead is where we can push in a lot of extra stuff
				- title
				- description
				- link ref=""
				- meta charset for amp (see https://www.ampproject.org/docs/reference/spec)
				- AMP boilerplate
				- stylesheet links or embedded CSS
				- Open Graph
				- Twitter Cards
				- schema.org
				- etc
		*/
		
		let mut nodesForHtmlHead = NodesForOtherPlacesInHtml::new();
		let htmlFromMarkdown = self.markdownParser.parse(&self.markdown, &mut nodesForHtmlHead, &self.markdownPluginData, isForAmp)?;
		
		let (headHtml, hiddenBodyHtml) = nodesForHtmlHead.headAndHiddenBodyHtml();
		
		let html =
		{
			let iso639Dash1Alpha2Language = self.iso639Dash1Alpha2Language;
			let imageResourceReference = match self.articleImage
			{
				None => None,
				Some((ref imageResourceReference, _)) => Some(imageResourceReference),
			};
			let imageAbstract = match self.articleImage
			{
				None => None,
				Some((_, ref imageMetaData)) => Some((imageResourceReference, imageMetaData.imageAbstract(iso639Dash1Alpha2Language)?)),
			};
			
			// TODO: Needs tidy-up into a form that is JSON / handlebars friendly
			
			// TODO: Dates need to be rendered in a form that is friendly to their usage.
			
			handlebars.renderHtmlUsingNamedTemplate(handlebarsTemplate, &json!
			({
				"is_for_amp": isForAmp,
				"environment": &self.configuration.environment,
				"iso639Dash1Alpha2Language": iso639Dash1Alpha2Language,
				"localization": &self.configuration.localization,
				"deployment_date": self.configuration.deploymentDate,
				"deployment_version": &self.configuration.deploymentVersion,
				"html_from_markdown": htmlFromMarkdown,
				"publication_date": self.publicationDate,
				"lastModificationDateOrPublicationDate": self.lastModificationDateOrPublicationDate,
				"modifications": self.modifications,
				"expiration_date": self.expirationDate,
				"document_abstract": self.htmlAbstract,
				"image_abstract": imageAbstract,
				"image_article": imageResourceReference,
				"site_map_images": self.siteMapImages,
				"headHtml": headHtml,
				"hiddenBodyHtml": hiddenBodyHtml,
			}))?
		};
		
		let document = RcDom::from_bytes_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type(html.as_bytes(), inputContentFilePath)?;
		let body = document.minify_to_bytes(true);
		Ok((document, body))
	}
}
