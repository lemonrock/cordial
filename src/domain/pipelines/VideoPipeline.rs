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
	
	#[serde(default)] pub(crate) metadata: Rc<AudioVideoMetaData>,
	
	#[serde(default, skip_deserializing)] pub(crate) dimensions: Cell<(u16, u16)>,
	#[serde(default, skip_deserializing)] pub(crate) durationInSeconds: Cell<u64>,
	#[serde(default, skip_deserializing)] pub(crate) mp4Url: RefCell<Option<Url>>,
	#[serde(default, skip_deserializing)] pub(crate) webmUrl: RefCell<Option<Url>>,
	#[serde(default, skip_deserializing)] pub(crate) orderedMapOfWebVttUrls: RefCell<OrderMap<(AudioVideoTrackKind, Iso639Dash1Alpha2Language), Url>>,
}

impl Default for VideoPipeline
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			max_age_in_seconds: max_age_in_seconds_long_default(),
			is_versioned: is_versioned_true_default(),
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
			
			metadata: Default::default(),
			
			dimensions: Default::default(),
			durationInSeconds: Default::default(),
			mp4Url: Default::default(),
			webmUrl: Default::default(),
			orderedMapOfWebVttUrls: Default::default(),
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
		(self.is_versioned, true)
	}
	
	#[inline(always)]
	fn execute(&self, resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, _handlebars: &HandlebarsWrapper, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, configuration: &Configuration, _rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, _siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<PipelineResource>, CordialError>
	{
		let isPrimaryLanguage = configuration.fallbackIso639Dash1Alpha2Language() == languageData.iso639Dash1Alpha2Language;
		
		let mut result = Vec::new();
		
		let (mp4Body, width, height, durationInSeconds) = if isPrimaryLanguage
		{
			let mp4Url = self.mp4Url(resourceUrl, configuration)?;
			*self.mp4Url.borrow_mut() = Some(mp4Url);
			
			let webmUrl = self.webmUrl(resourceUrl, configuration)?;
			*self.webmUrl.borrow_mut() = Some(webmUrl);
			
			let mp4Body = inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?;
			
			let (width, height, durationInSeconds) = firstVideoTrackDurationWidthAndHeight(&mp4Body)?;
			self.dimensions.set((width, height));
			self.durationInSeconds.set(durationInSeconds);
			
			(Some(mp4Body), width, height, durationInSeconds)
		}
		else
		{
			let (width, height) = self.dimensions.get();
			(None, width, height, self.durationInSeconds.get())
		};
		
		let videoNode =
		{
			let mp4UrlBorrow = self.mp4Url.borrow();
			let webmUrlBorrow = self.webmUrl.borrow();
			self.createVideoNode(resources, configuration, languageData, width, height, mp4UrlBorrow.as_ref().unwrap(), &webmUrlBorrow.as_ref().unwrap(), durationInSeconds)?
		};
		
		if isPrimaryLanguage
		{
			self.createWebVttTracks(inputContentFilePath, resourceUrl, configuration, headerGenerator, &mut result)?;
			self.createMp4(width, height, durationInSeconds, self.mp4Url.borrow().as_ref().unwrap().clone(), headerGenerator, mp4Body.unwrap(), &mut result)?;
			self.createWebm(width, height, durationInSeconds, self.mp4Url.borrow().as_ref().unwrap().clone(), headerGenerator, inputContentFilePath, &mut result)?;
		}
		
		self.createIFramePlayer(resourceUrl, videoNode, width, languageData, headerGenerator, &mut result)?;
		
		Ok(result)
	}
}

impl VideoPipeline
{
	pub(crate) fn siteMapWebPageVideo(&self, resourceUrl: &ResourceUrl, languageData: &LanguageData, configuration: &Configuration) -> Result<SiteMapWebPageAudioVideo, CordialError>
	{
		Ok
		(
			SiteMapWebPageAudioVideo
			{
				placeHolderUrl: self.placeholder.clone(),
				durationInSeconds: Some(self.durationInSeconds.get()),
				mediaUrl: self.mp4Url(resourceUrl, configuration)?,
				iFrameUrl: self.iFramePlayerUrl(resourceUrl, languageData)?,
				
				audioVideoMetaData: self.metadata.clone(),
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
	fn mp4Url(&self, resourceUrl: &ResourceUrl, configuration: &Configuration) -> Result<Url, CordialError>
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
					self.orderedMapOfWebVttUrls.borrow_mut().insert((track.kind, languageData.iso639Dash1Alpha2Language), webVttUrl.clone());
					
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
	fn createMp4(&self, width: u16, height: u16, durationInSeconds: u64, mp4Url: Url, headerGenerator: &mut HeaderGenerator, mp4Body: Vec<u8>, result: &mut Vec<PipelineResource>) -> Result<(), CordialError>
	{
		const Incompressible: bool = false;
		
		let mp4Headers = headerGenerator.generateHeadersForAsset(Incompressible, self.max_age_in_seconds, self.isDownloadable(), &mp4Url)?;
		let mp4Tags = hashmap!
		{
			video_mp4 => Rc::new(UrlDataDetails::video(&mp4Body, width, height, durationInSeconds))
		};
		result.push((mp4Url, mp4Tags, StatusCode::Ok, ContentType(mimeType(Self::Mp4TwitterMimeType)), mp4Headers, mp4Body, None, Incompressible));
		Ok(())
	}
	
	#[inline(always)]
	fn createWebm(&self, width: u16, height: u16, durationInSeconds: u64, webmUrl: Url, headerGenerator: &mut HeaderGenerator, inputContentFilePath: &Path, result: &mut Vec<PipelineResource>) -> Result<(), CordialError>
	{
		const Incompressible: bool = false;
		
		let webmInputContentFilePath = inputContentFilePath.with_extension("webm");
		let webmBody = webmInputContentFilePath.fileContentsAsBytes().context(webmInputContentFilePath)?;
		let webmHeaders = headerGenerator.generateHeadersForAsset(Incompressible, self.max_age_in_seconds, self.isDownloadable(), &webmUrl)?;
		let webmTags = hashmap!
		{
			video_webm => Rc::new(UrlDataDetails::video(&webmBody, width, height, durationInSeconds))
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
	fn audioVideoAbstract(&self, configuration: &Configuration, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&AudioVideoAbstract, CordialError>
	{
		self.metadata.audioVideoAbstract(configuration.fallbackIso639Dash1Alpha2Language(), iso639Dash1Alpha2Language)
	}
	
	#[inline(always)]
	fn title(&self, configuration: &Configuration, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&str, CordialError>
	{
		Ok(&self.audioVideoAbstract(configuration, iso639Dash1Alpha2Language)?.title)
	}
	
	#[inline(always)]
	pub(crate) fn videoNode(&self, isForAmp: bool, resources: &Resources, configuration: &Configuration, languageData: &LanguageData) -> Result<UnattachedNode, CordialError>
	{
		let (width, height) = self.dimensions.get();
		
		let mp4UrlBorrow = self.mp4Url.borrow();
		
		let webmUrlBorrow = self.webmUrl.borrow();
		
		let durationInSeconds = self.durationInSeconds.get();
		
		if isForAmp
		{
			self.createAmpVideoNode(resources, configuration, languageData, width, height, &mp4UrlBorrow.as_ref().unwrap(), &webmUrlBorrow.as_ref().unwrap(), durationInSeconds)
		}
		else
		{
			self.createVideoNode(resources, configuration, languageData, width, height, &mp4UrlBorrow.as_ref().unwrap(), &webmUrlBorrow.as_ref().unwrap(), durationInSeconds)
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn createAmpVideoNode(&self, resources: &Resources, configuration: &Configuration, languageData: &LanguageData, width: u16, height: u16, mp4Url: &Url, webmUrl: &Url, durationInSeconds: u64) -> Result<UnattachedNode, CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		let title = self.title(configuration, iso639Dash1Alpha2Language)?;
		
		let placeHolderUrlData = ResourceReference
		{
			resource: self.placeholder.clone(),
			tag: ResourceTag::width_height_image(width as u32, height as u32)
		}.urlDataMandatory(resources, configuration.fallbackIso639Dash1Alpha2Language(), Some(iso639Dash1Alpha2Language))?;
		placeHolderUrlData.validateIsPng()?;
		
		let mut ampVideoNode =
			"amp-video"
			.with_attributes
			(
				vec!
				[
					"width".string_attribute(format!("{}", width)),
					"height".string_attribute(format!("{}", height)),
					"title".str_attribute(title),
					"poster".str_attribute(placeHolderUrlData.url_str()),
					"layout".str_attribute("responsive"),
				]
			)
		;
		
		ampVideoNode = self.load.addToVideoNode(ampVideoNode, durationInSeconds);
		
		if self.load == AudioVideoLoad::auto_play
		{
			ampVideoNode = ampVideoNode.with_empty_attribute("autoplay");
		}
		
		if self.loops
		{
			ampVideoNode = ampVideoNode.with_empty_attribute("loop");
		}
		
		if self.show_controls
		{
			ampVideoNode = ampVideoNode.with_empty_attribute("controls");
			
			if !self.disabled_controls.is_empty()
			{
				ampVideoNode = ampVideoNode.with_attribute("controlslist".space_separated_attribute(self.disabled_controls.iter().map(|disabled_control| disabled_control.deref())));
			}
		}
		
		if self.disabled_controls.contains(&AudioVideoDisabledControl::noremoteplayback)
		{
			ampVideoNode = ampVideoNode.with_empty_attribute("disableremoteplayback");
		}
		
		if let Some(ref artist) = self.metadata.artist
		{
			ampVideoNode = ampVideoNode.with_attribute("artist".str_attribute(artist));
		}
		
		if let Some(ref album) = self.metadata.album
		{
			ampVideoNode = ampVideoNode.with_attribute("album".str_attribute(album));
		}
		
		if let Some(ref artwork) = self.metadata.artwork
		{
			// 256x256 or 512x512? Whilst artwork is an array, it's not clear if amp-video supports it.
			const ArtworkWidth: u32 = 512;
			const ArtworkHeight: u32 = 512;
			
			let artworkUrlData = ResourceReference
			{
				resource: artwork.clone(),
				tag: ResourceTag::width_height_image(ArtworkWidth, ArtworkHeight)
			}.urlDataMandatory(resources, configuration.fallbackIso639Dash1Alpha2Language(), Some(iso639Dash1Alpha2Language))?;
			artworkUrlData.validateIsPng()?;
			
			ampVideoNode = ampVideoNode.with_attribute("artwork".str_attribute(artworkUrlData.url_str()));
		}
		
		ampVideoNode = ampVideoNode
			.with_child_element
			(
				"noscript"
				.with_child_element(self.createVideoNode(resources, configuration, languageData, width, height, mp4Url, webmUrl, durationInSeconds)?)
			)
			.with_child_element
			(
				"div"
				.with_empty_attribute("fallback")
				.with_child_text(languageData.requiredTranslation(RequiredTranslation::missing_video_fallback)?.as_str())
			);
		
		self.addSourcesAndTracks(ampVideoNode, mp4Url, webmUrl, iso639Dash1Alpha2Language, configuration)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn createVideoNode(&self, resources: &Resources, configuration: &Configuration, languageData: &LanguageData, width: u16, height: u16, mp4Url: &Url, webmUrl: &Url, durationInSeconds: u64) -> Result<UnattachedNode, CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		let title = self.title(configuration, iso639Dash1Alpha2Language)?;
		
		let placeHolderUrlData = ResourceReference
		{
			resource: self.placeholder.clone(),
			tag: ResourceTag::width_height_image(width as u32, height as u32)
		}.urlDataMandatory(resources, configuration.fallbackIso639Dash1Alpha2Language(), Some(iso639Dash1Alpha2Language))?;
		placeHolderUrlData.validateIsPng()?;
		
		let mut videoNode =
			"video"
			.with_attributes
			(
				vec!
				[
					"width".string_attribute(format!("{}", width)),
					"height".string_attribute(format!("{}", height)),
					"title".str_attribute(title),
					"poster".str_attribute(placeHolderUrlData.url_str()),
				]
			)
		;
		
		videoNode = self.load.addToVideoNode(videoNode, durationInSeconds);
		
		// Twitter Player Card Rules: Default to ‘sound off’ for videos that automatically play content
		if self.initially_muted || self.load == AudioVideoLoad::auto_play
		{
			videoNode = videoNode.with_empty_attribute("muted");
		}
		
		if self.plays_inline
		{
			videoNode = videoNode.with_empty_attribute("playsinline");
		}
		
		if self.loops
		{
			videoNode = videoNode.with_empty_attribute("loop");
		}
		
		if self.show_controls
		{
			videoNode = videoNode.with_empty_attribute("controls");
			
			if !self.disabled_controls.is_empty()
			{
				videoNode = videoNode.with_attribute("controlsList".space_separated_attribute(self.disabled_controls.iter().map(|disabled_control| disabled_control.deref())));
			}
		}
		
		videoNode = self.addSourcesAndTracks(videoNode, mp4Url, webmUrl, iso639Dash1Alpha2Language, configuration)?;
		
		let translation = languageData.requiredTranslation(RequiredTranslation::your_browser_does_not_support_video)?;
		videoNode = videoNode.with_child_text(translation.deref().as_str());
		
		Ok(videoNode)
	}
	
	fn addSourcesAndTracks(&self, mut videoNode: UnattachedNode, mp4Url: &Url, webmUrl: &Url, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, configuration: &Configuration) -> Result<UnattachedNode, CordialError>
	{
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
		
		videoNode = videoNode.with_child_element
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
		
		let mut isDefaultTrackAndVideoNode = self.addTracksForLanguage((true, videoNode), iso639Dash1Alpha2Language)?;
		for (iso639Dash1Alpha2Language, _language) in configuration.otherLanguages(iso639Dash1Alpha2Language).iter()
		{
			isDefaultTrackAndVideoNode = self.addTracksForLanguage(isDefaultTrackAndVideoNode, *iso639Dash1Alpha2Language)?;
		}
		Ok(isDefaultTrackAndVideoNode.1)
	}
	
	fn addTracksForLanguage(&self, isFirstAndVideoNode: (bool, UnattachedNode), iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(bool, UnattachedNode), CordialError>
	{
		let (mut isDefaultTrack, mut videoNode) = isFirstAndVideoNode;
		
		let orderedMapOfWebVttUrls = self.orderedMapOfWebVttUrls.borrow();
		
		for track in self.tracks.iter()
		{
			if let Some(webVttUrl) = orderedMapOfWebVttUrls.get(&(track.kind, iso639Dash1Alpha2Language))
			{
				let trackNode = track.asNode(isDefaultTrack, iso639Dash1Alpha2Language, &webVttUrl)?;
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
	fn show_controls_default() -> bool
	{
		true
	}
}
