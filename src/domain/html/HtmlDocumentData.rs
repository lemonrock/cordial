// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct HtmlDocumentData<'a>
{
	pub(crate) resourceUrl: &'a ResourceUrl,
	pub(crate) fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language,
	pub(crate) markdownParser: MarkdownParser,
	pub(crate) markdown: String,
	pub(crate) markdownPluginData: MarkdownPluginData<'a>,
	pub(crate) htmlAbstract: Rc<HtmlAbstract>,
	pub(crate) articleImage: Option<(ResourceUrl, Rc<ImageMetaData>)>,
	pub(crate) siteMapImages: &'a [ResourceUrl],
	pub(crate) siteMapAudios: &'a [ResourceUrl],
	pub(crate) siteMapVideos: &'a [ResourceUrl],
	pub(crate) publicationDate: Option<DateTime<Utc>>,
	pub(crate) lastModificationDateOrPublicationDate: Option<DateTime<Utc>>,
	pub(crate) modifications: BTreeMap<DateTime<Utc>, Rc<String>>,
	pub(crate) expirationDate: Option<DateTime<Utc>>,
	pub(crate) configuration: &'a Configuration,
	pub(crate) htmlUrls: HtmlUrls<'a>,
	pub(crate) facebookOpenGraph: Rc<FacebookOpenGraph>,
	pub(crate) twitterCard: Rc<TwitterCard>,
	pub(crate) themeCssColor: Option<Rc<String>>,
	pub(crate) automaticTelephoneNumberDetection: bool,
	pub(crate) favIcon: Option<&'a FavIcon>,
	pub(crate) svgFavIcon: Option<&'a ResourceUrl>,
	pub(crate) safariStyling: Option<&'a SafariStyling>,
	pub(crate) windowsTilesBrowserConfig: Option<&'a ResourceUrl>,
}

impl<'a> HtmlDocumentData<'a>
{
	#[inline(always)]
	pub(crate) fn htmlTitle(&self) -> &str
	{
		&self.htmlAbstract.title_html
	}
	
	#[inline(always)]
	fn urlDataMandatory(&self, resources: &Resources, resourceUrl: &ResourceUrl, resourceTag: ResourceTag) -> Result<Rc<UrlData>, CordialError>
	{
		ResourceReference
		{
			resource: resourceUrl.clone(),
			tag: resourceTag,
		}.urlDataMandatory(resources, self.configuration.fallbackIso639Dash1Alpha2Language(), Some(self.iso639Dash1Alpha2Language()))
	}
	
	#[inline(always)]
	pub(crate) fn addStartHeadNodes(&self, startHeadNodes: &mut Vec<UnattachedNode>, _resources: &Resources) -> Result<(), CordialError>
	{
		if let Some(ref themeCssColor) = self.themeCssColor
		{
			startHeadNodes.push(meta_with_name_and_content("theme-color", themeCssColor.as_str()));
		}
		
		if !self.automaticTelephoneNumberDetection
		{
			startHeadNodes.push(meta_with_name_and_content("format-detection", "telephone=no"));
		}
		
		if let Some(safariStyling) = self.safariStyling
		{
			safariStyling.addToStartHeadNodes(startHeadNodes, &self.htmlAbstract);
		}
		
		Ok(())
		
		
		/*
			TODO: amp-manifest
				<link rel="amp-manifest" href="{{- $.Site.Params.ampManifest -}}">
			TODO: style amp-custom
				<style amp-custom>{{- partialCached "style.css" . -}}</style>
		*/
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn endHeadNodes(&self, addAmpLink: bool, ampLinkIsCanonical: bool, resources: &Resources) -> Result<Vec<UnattachedNode>, CordialError>
	{
		let mut endHeadNodes = vec!
		[
			meta_with_name_and_content("description", self.htmlDescription()),
		];
		
		if let Some(keywordsConcatenatedForBaidu) = self.keywordsConcatenatedForBaidu()
		{
			endHeadNodes.push(meta_with_name_and_content("keywords", keywordsConcatenatedForBaidu.as_str()))
		}
		
		self.addLinkNodes(&mut endHeadNodes, addAmpLink, ampLinkIsCanonical)?;
		
		if let Some(favIcon) = self.favIcon
		{
			favIcon.addLinkNodes(&mut endHeadNodes, "icon", resources, self.configuration.fallbackIso639Dash1Alpha2Language(), Some(self.iso639Dash1Alpha2Language()))?;
		}
		
		if let Some(svgFavIcon) = self.svgFavIcon
		{
			let urlData= self.urlDataMandatory(resources, svgFavIcon, ResourceTag::default)?;
			urlData.validateIsSvg()?;
			
			endHeadNodes.push("link".with_rel_attribute("icon").with_attribute("type".str_attribute(urlData.mimeType().as_ref())).with_href_attribute(urlData.url_str()))
		}
		
		if let Some(safariStyling) = self.safariStyling
		{
			safariStyling.addToEndHeadNodes(&mut endHeadNodes, resources, self.configuration.fallbackIso639Dash1Alpha2Language(), Some(self.iso639Dash1Alpha2Language()), self.resourceUrl)?;
		}
		
		if let Some(ref windowsTilesTitle) = self.htmlAbstract.windows_tiles_title
		{
			endHeadNodes.push(meta_with_name_and_content("application-name", windowsTilesTitle.as_str()));
		}
		
		if let Some(ref resourceUrl) = self.windowsTilesBrowserConfig
		{
			let urlData = self.urlDataMandatory(resources, resourceUrl, ResourceTag::default)?;
			urlData.validateIsXml()?;
			endHeadNodes.push(meta_with_name_and_content("msapplication-config", urlData.url_str()));
		}
		else
		{
			endHeadNodes.push(meta_with_name_and_content("msapplication-config", "none"));
		}
		
		self.addFacebookOpenGraphHtmlNodes(&mut endHeadNodes,  resources)?;
		
		self.addTwitterCardHtmlNodes(&mut endHeadNodes, resources)?;
		
		Ok(endHeadNodes)
	}
	
	#[inline(always)]
	fn htmlDescription(&self) -> &str
	{
		self.description()
	}
	
	#[inline(always)]
	fn keywordsConcatenatedForBaidu(&self) -> Option<String>
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
	fn addLinkNodes(&self, endHeadNodes: &mut Vec<UnattachedNode>, addAmpLink: bool, ampLinkIsCanonical: bool) -> Result<(), CordialError>
	{
		self.htmlUrls.addLinkNodes(endHeadNodes, addAmpLink, ampLinkIsCanonical)
	}
	
	#[inline(always)]
	fn addFacebookOpenGraphHtmlNodes(&self, endHeadNodes: &mut Vec<UnattachedNode>, resources: &Resources) -> Result<(), CordialError>
	{
		self.facebookOpenGraph.facebookOpenGraph(endHeadNodes, self.title(), self.description(), &self.htmlUrls.linkHeaderCanonicalUrl()?, self.publicationDate, self.lastModificationDateOrPublicationDate, self.expirationDate, self.configuration, resources, &self.articleImage, self.fallbackIso639Dash1Alpha2Language, self.htmlUrls.languageData)
	}
	
	#[inline(always)]
	fn addTwitterCardHtmlNodes(&self, endHeadNodes: &mut Vec<UnattachedNode>, resources: &Resources) -> Result<(), CordialError>
	{
		let articleAudio = self.siteMapAudios.get(0);
		let articleVideo = self.siteMapVideos.get(0);
		
		self.twitterCard.addTo(endHeadNodes, &self.articleImage, articleAudio, articleVideo, resources, self.fallbackIso639Dash1Alpha2Language, self.htmlUrls.languageData)
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
	pub(crate) fn iso639Dash1Alpha2Language(&self) -> Iso639Dash1Alpha2Language
	{
		self.htmlUrls.languageData.iso639Dash1Alpha2Language
	}
	
	#[inline(always)]
	pub(crate) fn addToRssChannels(&self, resources: &Resources, rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, rss: &Option<Rc<HtmlDocumentItem>>, rssChannels: &OrderMap<Rc<RssChannelName>, ()>, inputContentFilePath: &Path, handlebars: &HandlebarsWrapper) -> Result<(), CordialError>
	{
		if let &Some(ref rss) = rss
		{
			let iso639Dash1Alpha2Language = self.htmlUrls.languageData.iso639Dash1Alpha2Language;
			let canonicalLinkUrl = Rc::new(self.htmlUrls.linkHeaderCanonicalUrl()?);
			let description = &self.htmlAbstract.description;
			let lastModificationDate = self.lastModificationDateOrPublicationDate;
			
			for (rssChannelName, _) in rssChannels.iter()
			{
				match rssChannelsToRssItems.get_mut(rssChannelName)
				{
					None => return Err(CordialError::Configuration(format!("RSS channel '{}' does not have a configuration", rssChannelName))),
					Some(mut rssItems) =>
					{
						const IsNotForAmp: bool = false;
						let (_document, rssHtml) = self.renderHtmlDocument(resources, false, inputContentFilePath, IsNotForAmp, IsNotForAmp, IsNotForAmp, handlebars, rssChannelName)?;
						rss.withRssHtml(description.clone(), rssHtml, iso639Dash1Alpha2Language)?;
						rssItems.push
						(
							RssItem
							{
								canonicalLinkUrl: canonicalLinkUrl.clone(),
								lastModificationDate,
								htmlDocumentItem: rss.clone(),
							}
						);
					}
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
			images.push(articleImageMetaData.siteMapWebPageImage(siteMapImage, self.fallbackIso639Dash1Alpha2Language, self.iso639Dash1Alpha2Language())?);
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
			images.push(imageMetaData.siteMapWebPageImage(siteMapImage, self.fallbackIso639Dash1Alpha2Language, self.iso639Dash1Alpha2Language())?)
		}
		
		let mut audiosVideos = vec![];
		
		for siteMapAudioResourceUrl in self.siteMapAudios.iter()
		{
			let resourceRef = siteMapAudioResourceUrl.resourceMandatory(resources)?;
			let audioPipeline = resourceRef.audioPipeline()?;
			audiosVideos.push(audioPipeline.siteMapWebPageAudio(siteMapAudioResourceUrl, self.htmlUrls.languageData, self.configuration)?);
		}
		
		for siteMapVideoResourceUrl in self.siteMapVideos.iter()
		{
			let resourceRef = siteMapVideoResourceUrl.resourceMandatory(resources)?;
			let videoPipeline = resourceRef.videoPipeline()?;
			audiosVideos.push(videoPipeline.siteMapWebPageVideo(siteMapVideoResourceUrl, self.htmlUrls.languageData, self.configuration)?);
		}
		
		siteMapWebPages.push
		(
			SiteMapWebPage
			{
				lastModified: self.lastModificationDateOrPublicationDate,
				changeFrequency,
				priority,
				urlsByIso639Dash1Alpha2Language: self.htmlUrls.linkHeaderAlternativeLanguageUrlsIncludingSelf()?,
				images,
				audiosVideos,
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
