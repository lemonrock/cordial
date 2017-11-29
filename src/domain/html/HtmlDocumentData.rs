// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct HtmlDocumentData<'a>
{
	pub(crate) fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language,
	pub(crate) markdownParser: MarkdownParser,
	pub(crate) markdown: String,
	pub(crate) markdownPluginData: MarkdownPluginData<'a>,
	pub(crate) htmlAbstract: Rc<HtmlAbstract>,
	pub(crate) articleImage: Option<(ResourceUrl, Rc<ImageMetaData>)>,
	pub(crate) siteMapImages: &'a [ResourceUrl],
	pub(crate) publicationDate: Option<DateTime<Utc>>,
	pub(crate) lastModificationDateOrPublicationDate: Option<DateTime<Utc>>,
	pub(crate) modifications: BTreeMap<DateTime<Utc>, Rc<String>>,
	pub(crate) expirationDate: Option<DateTime<Utc>>,
	pub(crate) configuration: &'a Configuration,
	pub(crate) htmlUrls: HtmlUrls<'a>,
	pub(crate) facebookOpenGraph: Rc<FacebookOpenGraph>,
	pub(crate) twitterCard: Rc<TwitterCard>,
	pub(crate) themeCssColor: Option<Rc<String>>,
}

impl<'a> HtmlDocumentData<'a>
{
	#[inline(always)]
	pub(crate) fn addFacebookOpenGraphHtmlNodes(&self, endHeadNodes: &mut Vec<UnattachedNode>, resources: &Resources) -> Result<(), CordialError>
	{
		self.facebookOpenGraph.facebookOpenGraph(endHeadNodes, self.title(), self.description(), &self.htmlUrls.linkHeaderCanonicalUrl()?, self.publicationDate, self.lastModificationDateOrPublicationDate, self.expirationDate, self.configuration, resources, &self.articleImage, self.fallbackIso639Dash1Alpha2Language, self.htmlUrls.languageData)
	}
	
	#[inline(always)]
	pub(crate) fn addTwitterCardHtmlNodes(&self, endHeadNodes: &mut Vec<UnattachedNode>, resources: &Resources) -> Result<(), CordialError>
	{
		self.twitterCard.addTo(endHeadNodes, &self.articleImage, resources, self.fallbackIso639Dash1Alpha2Language, self.htmlUrls.languageData)
	}
	
	#[inline(always)]
	pub(crate) fn addLinkNodes(&self, endHeadNodes: &mut Vec<UnattachedNode>, addAmpLink: bool, ampLinkIsCanonical: bool) -> Result<(), CordialError>
	{
		self.htmlUrls.addLinkNodes(endHeadNodes, addAmpLink, ampLinkIsCanonical)
	}
	
	#[inline(always)]
	pub(crate) fn htmlTitle(&self) -> &str
	{
		&self.htmlAbstract.htmlTitle
	}
	
	#[inline(always)]
	pub(crate) fn htmlDescription(&self) -> &str
	{
		self.description()
	}
	
	#[inline(always)]
	pub(crate) fn title(&self) -> &str
	{
		&self.htmlAbstract.title
	}
	
	#[inline(always)]
	pub(crate) fn description(&self) -> &str
	{
		&self.htmlAbstract.description
	}
	
	#[inline(always)]
	pub(crate) fn keywordsConcatenatedForBaidu(&self) -> Option<String>
	{
		let keywordsForBaidu = &self.htmlAbstract.keywords_for_baidu;
		if keywordsForBaidu.is_empty()
		{
			return None;
		}
		
		let mut concatenatedKeywords = String::new();
		let mut isAfterFirst = false;
		for keyword in keywordsForBaidu.iter()
		{
			if isAfterFirst
			{
				concatenatedKeywords.push(',');
			}
			else
			{
				isAfterFirst = true;
			}
			concatenatedKeywords.push_str(keyword);
		}
		
		Some(concatenatedKeywords)
	}
	
	#[inline(always)]
	pub(crate) fn themeCssColor(&self) -> Option<&Rc<String>>
	{
		self.themeCssColor.as_ref()
	}
	
	#[inline(always)]
	pub(crate) fn iso639Dash1Alpha2Language(&self) -> Iso639Dash1Alpha2Language
	{
		self.htmlUrls.languageData.iso639Dash1Alpha2Language
	}
	
	#[inline(always)]
	pub(crate) fn addToRssChannels(&self, resources: &Resources, rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, author: &Rc<EMailAddress>, rssChannelsToCategories: &OrderMap<Rc<RssChannelName>, Rc<BTreeSet<RssCategoryName>>>, inputContentFilePath: &Path, handlebars: &HandlebarsWrapper) -> Result<(), CordialError>
	{
		const RssImageResourceTag: ResourceTag = ResourceTag::largest_image;
		
		for (rssChannelName, categories) in rssChannelsToCategories.iter()
		{
			match rssChannelsToRssItems.get_mut(rssChannelName)
			{
				None => return Err(CordialError::Configuration(format!("RSS channel '{}' does not have a configuration", rssChannelName))),
				Some(mut rssItems) =>
				{
					const IsNotForAmp: bool = false;
					let (_document, rssHtml) = self.renderHtmlDocument(resources, false, inputContentFilePath, IsNotForAmp, IsNotForAmp, IsNotForAmp, handlebars, rssChannelName)?;
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
									Some((ref articleImageResourceUrl, ref articleImageMetaData)) =>
									{
										let rssImage = ResourceReference
										{
											resource: articleImageResourceUrl.clone(),
											tag: RssImageResourceTag,
										};
										Some(articleImageMetaData.rssImage(&rssImage, self.fallbackIso639Dash1Alpha2Language, self.iso639Dash1Alpha2Language(), resources)?)
									}
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
		const SiteMapImageTag: ResourceTag = ResourceTag::largest_image;
		
		let mut images = vec![];
		if let Some((ref articleImageResourceUrl, ref articleImageMetaData)) = self.articleImage
		{
			let siteMapImage = ResourceReference
			{
				resource: articleImageResourceUrl.clone(),
				tag: SiteMapImageTag,
			};
			images.push(articleImageMetaData.siteMapWebPageImage(&siteMapImage, self.fallbackIso639Dash1Alpha2Language, self.iso639Dash1Alpha2Language(), resources)?);
		};
		
		for siteMapImageResourceUrl in self.siteMapImages.iter()
		{
			let resourceRef = siteMapImageResourceUrl.resourceMandatory(resources)?;
			let imageMetaData = resourceRef.imageMetaData()?;
			
			let siteMapImage = ResourceReference
			{
				resource: siteMapImageResourceUrl.clone(),
				tag: SiteMapImageTag,
			};
			images.push(imageMetaData.siteMapWebPageImage(&siteMapImage, self.fallbackIso639Dash1Alpha2Language, self.iso639Dash1Alpha2Language(), resources)?)
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
	pub(crate) fn renderHtmlDocument(&self, resources: &Resources, pjaxIsSupported: bool, inputContentFilePath: &Path, isForAmp: bool, addAmpLink: bool, ampLinkIsCanonical: bool, handlebars: &HandlebarsWrapper, handlebarsTemplate: &str) -> Result<(RcDom, Vec<u8>), CordialError>
	{
		/*
			nodesForHtmlHead is where we can push in a lot of extra stuff
	
				- link ref="" for manifest, icon, theme-color, amp-manifest
				- AMP boilerplate for embedded stylesheet css / stylesheet links
				- schema.org
		*/
		
		
		/*
{{ define "SchemaOrgStructuredDataExtension" -}}
{
	"@type": "BlogPosting",
	"mainEntityOfPage": {{- .Permalink -}}",
	"headline": "{{- .Title -}}",
	"image":
	{
		"@type": "ImageObject",
		"url": "{{- .Params.image.src | absURL -}}",
		{{- with (imageConfig (replace (.Params.image.src | absURL) $.Site.BaseURL "static/")) -}}
			"height": {{ .Height }},
			"width": {{ .Width }}
		{{- end -}}
	},
	"publisher":
	{
		"name": "{{- $.Site.Data.Organization.name -}}",
		"logo":
		{
			"@type": "ImageObject"
			"url: "{{- $.Site.Data.Organization.logo -}}",
			{{- with (imageConfig (replace $.Site.Data.Organization.logo $.Site.BaseURL "static/")) -}}
				"height": {{ .Height }},
				"width": {{ .Width }}
			{{- end -}}
		}
	}
	,"datePublished: "{{- .PublishDate.Format "2006-01-02T15:04:05" -}}""
	{{- if not .Lastmod.IsZero -}}, "dateModified: "{{- .Lastmod.Format "2006-01-02T15:04:05" -}}"{{- else if not .PublishDate.IsZero -}}, "dateModified: "{{- .PublishDate.Format "2006-01-02T15:04:05" -}}{{- end -}}
	,"author":
	{
		"@type": "Person",
		"name": "{{- $.Param "author.display_name" -}}"
	},
	"description": "{{- $.Param "description" -}}"
}
{{- end -}}
		
		
		*/
		
		let mut nodesForHtmlHead = NodesForOtherPlacesInHtml::new(isForAmp, addAmpLink, ampLinkIsCanonical, pjaxIsSupported, self.configuration, self, resources)?;
		
		
		
		
		
		let htmlFromMarkdown = self.markdownParser.parse(&self.markdown, &mut nodesForHtmlHead, &self.markdownPluginData, isForAmp)?;
		
		let (headHtml, hiddenBodyHtml) = nodesForHtmlHead.headAndHiddenBodyHtml();
		
		let html =
		{
			let iso639Dash1Alpha2Language = self.iso639Dash1Alpha2Language();
			
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
				"image_article": match self.articleImage
				{
					None => None,
					Some((ref imageResourceUrl, ref imageMetaData)) =>
					{
						Some((imageResourceUrl, imageMetaData.imageAbstract(iso639Dash1Alpha2Language)?))
					},
				},
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