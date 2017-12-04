// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// See https://www.w3.org/TR/appmanifest/#webappmanifest-dictionary
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct VideoPipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default)] input_format: Option<VideoInputFormat>,
	
	#[serde(default)] pub(crate) load: AudioVideoLoad,
	#[serde(default)] pub(crate) initially_muted: bool,
	#[serde(default)] pub(crate) loops: bool,
	#[serde(default)] pub(crate) plays_inline: bool,
	#[serde(default)] pub(crate) placeholder: ResourceUrl,
	#[serde(default)] pub(crate) starts_at_seconds_inclusive: u32,
	#[serde(default)] pub(crate) ends_at_seconds_exclusive: Option<u32>,
	#[serde(default)] pub(crate) tracks: Vec<AudioVideoTrack>,
	#[serde(default = "VideoPipeline::show_controls_default")] pub(crate) show_controls: bool,
	#[serde(default)] pub(crate) disabled_controls: BTreeSet<AudioVideoDisabledControl>,
	
	// Used by amp-video, Google Video Site Map, ?twitter player card? (if we decide to)
	#[serde(default)] pub(crate) abstracts: HashMap<Iso639Dash1Alpha2Language, Rc<VideoAbstract>>,
	
	// Used by amp-video
	#[serde(default)] pub(crate) artist: Option<String>,
	#[serde(default)] pub(crate) album: Option<String>,
	#[serde(default)] pub(crate) artwork: Option<ResourceUrl>,
	
	// Used by site map
	#[serde(default)] pub(crate) expiration_date: Option<DateTime<Utc>>,
	#[serde(default)] pub(crate) publication_date: Option<DateTime<Utc>>,
	#[serde(default)] pub(crate) rating: Option<VideoStarRating>,
	#[serde(default)] pub(crate) views: Option<u64>,
	#[serde(default)] pub(crate) can_appear_in_safe_search: bool,
	#[serde(default)] pub(crate) country_restrictions: Rc<VideoCountryRestriction>,
	#[serde(default)] pub(crate) platform_restrictions: Rc<VideoPlatformRestriction>,
	#[serde(default)] pub(crate) gallery: Option<ResourceUrl>,
	#[serde(default)] pub(crate) requires_subscription: bool,
	#[serde(default)] pub(crate) uploader: Option<Rc<Person>>,

	#[serde(default, skip_deserializing)] pub(crate) dimensions: Cell<(u16, u16)>,
	#[serde(default, skip_deserializing)] pub(crate) durationInSeconds: Cell<u64>,
}

impl Default for VideoPipeline
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			max_age_in_seconds: max_age_in_seconds_long_default(),
			is_versioned: Self::language_aware_default(),
			input_format: None,
			
			load: Default::default(),
			initially_muted: false,
			loops: false,
			plays_inline: false,
			placeholder: Default::default(),
			starts_at_seconds_inclusive: 0,
			ends_at_seconds_exclusive: None,
			tracks: Default::default(),
			show_controls: Self::show_controls_default(),
			disabled_controls: Default::default(),
			abstracts: Default::default(),
			
			artist: None,
			album: None,
			artwork: None,
			
			expiration_date: None,
			publication_date: None,
			rating: None,
			views: None,
			can_appear_in_safe_search: false,
			country_restrictions: Default::default(),
			platform_restrictions: Default::default(),
			gallery: None,
			requires_subscription: false,
			uploader: None,
			
			dimensions: Default::default(),
			durationInSeconds: Default::default(),
		}
	}
}

impl Pipeline for VideoPipeline
{
	#[inline(always)]
	fn processingPriority(&self) -> ProcessingPriority
	{
		DependsOnOthersEgStylesheetOrVideo
	}
	
	#[inline(always)]
	fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		self.input_format.resourceInputContentFileNamesWithExtension(resourceInputName)
	}
	
	#[inline(always)]
	fn is<'a>(&self) -> (bool, bool)
	{
		(self.is_versioned, false)
	}
	
	#[inline(always)]
	fn execute(&self, resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, _handlebars: &HandlebarsWrapper, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, configuration: &Configuration, _rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, _siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<PipelineResource>, CordialError>
	{
		let isPrimaryLanguage = configuration.fallbackIso639Dash1Alpha2Language() == languageData.iso639Dash1Alpha2Language;
		
		let mp4Url = self.mp4Url(resourceUrl, configuration)?;
		let webmUrl = self.webmUrl(resourceUrl, configuration)?;
		
		let mut result = Vec::new();
		
		let (mp4Body, width, height) = if isPrimaryLanguage
		{
			let mp4Body = inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?;
			let (width, height, durationInSeconds) = firstVideoTrackDurationWidthAndHeight(&mp4Body)?;
			self.dimensions.set((width, height));
			self.durationInSeconds.set(durationInSeconds);
			(Some(mp4Body), width, height)
		}
		else
		{
			let (width, height) = self.dimensions.get();
			(None, width, height)
		};
		
		let videoNode = self.createVideoNode(resourceUrl, resources, configuration, languageData, inputContentFilePath, width, height, &mp4Url, &webmUrl, self.durationInSeconds.get())?;
		
		if isPrimaryLanguage
		{
			self.createWebVttTracks(inputContentFilePath, resourceUrl, configuration, headerGenerator, &mut result)?;
			self.createMp4(width, height, mp4Url, headerGenerator, mp4Body.unwrap(), &mut result)?;
			self.createWebm(width, height, webmUrl, headerGenerator, inputContentFilePath, &mut result)?;
		}
		
		self.createIFramePlayer(resourceUrl, videoNode, width, languageData, headerGenerator, &mut result)?;
		
		Ok(result)
	}
}

impl VideoPipeline
{
	pub(crate) fn siteMapWebPageVideo(&self, resourceUrl: &ResourceUrl, languageData: &LanguageData, configuration: &Configuration) -> Result<SiteMapWebPageVideo, CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		Ok
		(
			SiteMapWebPageVideo
			{
				placeHolderUrl: self.placeholder.clone(),
				
				videoAbstract: self.videoAbstract(iso639Dash1Alpha2Language)?.clone(),
				mp4Url: self.mp4Url(resourceUrl, configuration)?,
				iFrameUrl: self.iFramePlayerUrl(resourceUrl, languageData)?,
				durationInSeconds: Some(self.durationInSeconds.get()),
				
				expirationDate: self.expiration_date,
				videoStarRating: self.rating,
				viewCount: self.views,
				publicationDate: self.publication_date,
				canAppearInSafeSearch: self.can_appear_in_safe_search,
				countryRestrictions: self.country_restrictions.clone(),
				gallery: self.gallery.clone(),
				requiresSubscription: self.requires_subscription,
				uploader: self.uploader.clone(),
				platformRestrictions: self.platform_restrictions.clone(),
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn dimensions(&self) -> (u16, u16)
	{
		self.dimensions.get()
	}
	
	#[inline(always)]
	pub(crate) fn mp4ContentType(&self) -> &str
	{
		Self::Mp4TwitterMimeType
	}
	
	#[inline(always)]
	pub(crate) fn mp4Url(&self, resourceUrl: &ResourceUrl, configuration: &Configuration) -> Result<Url, CordialError>
	{
		resourceUrl.replaceFileNameExtension(".mp4").url(&configuration.primaryLanguageData()?)
	}
	
	#[inline(always)]
	fn webmUrl(&self, resourceUrl: &ResourceUrl, configuration: &Configuration) -> Result<Url, CordialError>
	{
		resourceUrl.replaceFileNameExtension(".webm").url(&configuration.primaryLanguageData()?)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn iFramePlayerUrl(&self, resourceUrl: &ResourceUrl, languageData: &LanguageData) -> Result<Url, CordialError>
	{
		resourceUrl.replaceFileNameExtension(".iframe-player.html").url(languageData)
	}
	
	// See http://www.leanbackplayer.com/test/h5mt.html for most variants
	
	//noinspection SpellCheckingInspection
	const Mp4TwitterMimeType: &'static str = "video/mp4;codecs=\"avc1.42E01E,mp4a.40.2\"";
	
	//noinspection SpellCheckingInspection
	const WebMVp8MimeType: &'static str = "video/webm;codecs=\"vp8,vorbis\"";
	
	#[inline(always)]
	fn createWebVttTracks(&self, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, configuration: &Configuration, headerGenerator: &mut HeaderGenerator, result: &mut Vec<PipelineResource>) -> Result<(), CordialError>
	{
		for track in self.tracks.iter()
		{
			const CanBeCompressed: bool = true;
			
			configuration.visitLanguagesWithPrimaryFirst(|languageData, _isPrimaryLanguage|
			{
				if let Some((webVttBody, webVttUrl)) = track.bodyAndUrl(languageData, inputContentFilePath, resourceUrl)?
				{
					let webVttHeaders = headerGenerator.generateHeadersForAsset(CanBeCompressed, self.max_age_in_seconds, self.isDownloadable(), &webVttUrl)?;
					result.push((webVttUrl, hashmap! { video_track(track.kind, languageData.iso639Dash1Alpha2Language) => Rc::new(UrlDataDetails::generic(&webVttBody)) }, StatusCode::Ok, ContentType(mimeType("text/vtt")), webVttHeaders, webVttBody, None, CanBeCompressed));
				}
				
				Ok(())
			})?;
		}
		Ok(())
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn createIFramePlayer(&self, resourceUrl: &ResourceUrl, videoNode: UnattachedNode, width: u16, languageData: &LanguageData, headerGenerator: &mut HeaderGenerator, result: &mut Vec<PipelineResource>) -> Result<(), CordialError>
	{
		const Compressible: bool = true;
		
		let iFramePlayerUrl = self.iFramePlayerUrl(resourceUrl, languageData)?;
		let iFramePlayerBody = Self::iFramePlayerHtmlBody(videoNode, width);
		let iFramePlayerHeaders = headerGenerator.generateHeadersForAsset(Compressible, self.max_age_in_seconds, false, &iFramePlayerUrl)?;
		let iFramePlayerTags = hashmap!
		{
			video_iframe => Rc::new(UrlDataDetails::generic(&iFramePlayerBody))
		};
		result.push((iFramePlayerUrl, iFramePlayerTags, StatusCode::Ok, ContentType::html(), iFramePlayerHeaders, iFramePlayerBody, None, Compressible));
		Ok(())
	}
	
	#[inline(always)]
	fn createMp4(&self, width: u16, height: u16, mp4Url: Url, headerGenerator: &mut HeaderGenerator, mp4Body: Vec<u8>, result: &mut Vec<PipelineResource>) -> Result<(), CordialError>
	{
		const Incompressible: bool = false;
		
		let mp4Headers = headerGenerator.generateHeadersForAsset(Incompressible, self.max_age_in_seconds, self.isDownloadable(), &mp4Url)?;
		let mp4Tags = hashmap!
		{
			video_mp4 => Rc::new(UrlDataDetails::video(&mp4Body, width, height))
		};
		result.push((mp4Url, mp4Tags, StatusCode::Ok, ContentType(mimeType(Self::Mp4TwitterMimeType)), mp4Headers, mp4Body, None, Incompressible));
		Ok(())
	}
	
	#[inline(always)]
	fn createWebm(&self, width: u16, height: u16, webmUrl: Url, headerGenerator: &mut HeaderGenerator, inputContentFilePath: &Path, result: &mut Vec<PipelineResource>) -> Result<(), CordialError>
	{
		const Incompressible: bool = false;
		
		let webmInputContentFilePath = inputContentFilePath.with_extension("webm");
		let webmBody = webmInputContentFilePath.fileContentsAsBytes().context(webmInputContentFilePath)?;
		let webmHeaders = headerGenerator.generateHeadersForAsset(Incompressible, self.max_age_in_seconds, self.isDownloadable(), &webmUrl)?;
		let webmTags = hashmap!
		{
			video_webm => Rc::new(UrlDataDetails::video(&webmBody, width, height))
		};
		result.push((webmUrl, webmTags, StatusCode::Ok, ContentType(mimeType(Self::WebMVp8MimeType)), webmHeaders, webmBody, None, Incompressible));
		Ok(())
	}
	
	#[inline(always)]
	fn iFramePlayerHtmlBody(videoNode: UnattachedNode, width: u16) -> Vec<u8>
	{
		let htmlNode = "html"
		.with_child_element
		(
			"head"
			.with_child_element
			(
				"style"
				.with_type_attribute("text/css")
				.with_child_text(format!("width:100%;max-width:{};height:auto", width))
			)
			.with_child_element
			(
				meta_with_name_and_content("robots", X_Robots_Tag_Data)
			)
		)
		.with_child_element
		(
			"body"
			.with_child_element(videoNode)
		);
		
		htmlNode.to_html5_document(true).into_bytes()
	}

	#[inline(always)]
	fn videoAbstract(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&Rc<VideoAbstract>, CordialError>
	{
		match self.abstracts.get(&iso639Dash1Alpha2Language)
		{
			None => return Err(CordialError::Configuration(format!("There is no abstract in {:?} for this video", iso639Dash1Alpha2Language))),
			Some(videoAbstract) => Ok(videoAbstract),
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn createVideoNode(&self, resourceUrl: &ResourceUrl, resources: &Resources, configuration: &Configuration, languageData: &LanguageData, inputContentFilePath: &Path, width: u16, height: u16, mp4Url: &Url, webmUrl: &Url, durationInSeconds: u64) -> Result<UnattachedNode, CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		let title = &self.videoAbstract(iso639Dash1Alpha2Language)?.title;
		
		let mut videoNode =
			"video"
			.with_attributes
			(
				vec!
				[
					"width".string_attribute(format!("{}", width)),
					"height".string_attribute(format!("{}", height)),
					"title".str_attribute(title),
				]
			)
		;
		
		videoNode = self.load.addToVideoNode(videoNode, durationInSeconds);
		
		// Twitter Player Card Rules: Default to ‘sound off’ for videos that automatically play content
		if self.initially_muted || self.load == AudioVideoLoad::auto_play
		{
			videoNode = videoNode.with_empty_attribute("muted");
		}
		
		if self.loops
		{
			videoNode = videoNode.with_empty_attribute("loop");
		}
		
		if self.plays_inline
		{
			videoNode = videoNode.with_empty_attribute("playsinline");
		}
		
		let placeHolderUrlData = ResourceReference
		{
			resource: self.placeholder.clone(),
			tag: ResourceTag::width_height_image(width as u32, height as u32)
		}.urlDataMandatory(resources, configuration.fallbackIso639Dash1Alpha2Language(), Some(iso639Dash1Alpha2Language))?;
		placeHolderUrlData.validateIsPng()?;
		videoNode = videoNode.with_attribute("poster".str_attribute(placeHolderUrlData.url_str()));
		
		if self.show_controls
		{
			videoNode = videoNode.with_empty_attribute("controls");
			
			if !self.disabled_controls.is_empty()
			{
				videoNode = videoNode.with_attribute("controlsList".space_separated_attribute(self.disabled_controls.iter().map(|disabled_control| disabled_control.deref())));
			}
		}
		
		let mediaTimeFragment = match self.ends_at_seconds_exclusive
		{
			None => if self.starts_at_seconds_inclusive == 0
			{
				"".to_owned()
			}
			else
			{
				format!("#t={}", self.starts_at_seconds_inclusive)
			},
			Some(ends_at_seconds_exclusive) => if ends_at_seconds_exclusive <= self.starts_at_seconds_inclusive
			{
				return Err(CordialError::Configuration("ends_at_seconds_exclusive must be greater than starts_at_seconds_inclusive is specified".to_owned()));
			}
			else
			{
				if self.starts_at_seconds_inclusive == 0
				{
					format!("#t=,{}", ends_at_seconds_exclusive)
				}
				else
				{
					format!("#t={},{}", self.starts_at_seconds_inclusive, ends_at_seconds_exclusive)
				}
			},
		};
		
		videoNode = videoNode
			.with_child_element
			(
				"source"
				.with_type_attribute(Self::WebMVp8MimeType)
				.with_attribute("src".string_attribute(format!("{}{}", webmUrl.as_ref(), &mediaTimeFragment)))
			)
			.with_child_element
			(
				"source"
				.with_type_attribute(Self::Mp4TwitterMimeType)
				.with_attribute("src".string_attribute(format!("{}{}", mp4Url.as_ref(), &mediaTimeFragment)))
			)
		;
		
		let mut isDefaultTrackAndVideoNode = self.addTracksForLanguage((true, videoNode), languageData, inputContentFilePath, resourceUrl)?;
		for (iso639Dash1Alpha2Language, language) in configuration.otherLanguages(iso639Dash1Alpha2Language).iter()
		{
			let languageData = LanguageData { iso639Dash1Alpha2Language: *iso639Dash1Alpha2Language, language };
			isDefaultTrackAndVideoNode = self.addTracksForLanguage(isDefaultTrackAndVideoNode, &languageData, inputContentFilePath, resourceUrl)?;
		}
		videoNode = isDefaultTrackAndVideoNode.1;
		
		let translation = languageData.requiredTranslation(RequiredTranslation::your_browser_does_not_support_video)?;
		videoNode = videoNode.with_child_text(translation.deref().as_str());
		
		Ok(videoNode)
	}
	
	fn addTracksForLanguage(&self, isFirstAndVideoNode: (bool, UnattachedNode), languageData: &LanguageData, inputContentFilePath: &Path, resourceUrl: &ResourceUrl) -> Result<(bool, UnattachedNode), CordialError>
	{
		let (mut isDefaultTrack, mut videoNode) = isFirstAndVideoNode;
		
		for track in self.tracks.iter()
		{
			// TODO: Performance. We load the WebVtt data multiple times.
			// Do we have the track in this language? In theory, we could cache the webVttUrls as we process primary lang first, but it's not an obvious thing to do.
			if let Some((_webVttBody, webVttUrl)) = track.bodyAndUrl(languageData, inputContentFilePath, resourceUrl)?
			{
				let trackNode = track.asNode(isDefaultTrack, languageData.iso639Dash1Alpha2Language, &webVttUrl)?;
				videoNode = videoNode.with_child_element(trackNode);
				
				if isDefaultTrack
				{
					isDefaultTrack = false;
				}
			}
		}
		Ok((isDefaultTrack, videoNode))
	}
	
	#[inline(always)]
	fn isDownloadable(&self) -> bool
	{
		if self.disabled_controls.contains(&AudioVideoDisabledControl::nodownload)
		{
			false
		}
		else
		{
			true
		}
	}
	
	#[inline(always)]
	fn language_aware_default() -> bool
	{
		true
	}
	
	#[inline(always)]
	fn show_controls_default() -> bool
	{
		true
	}
}
