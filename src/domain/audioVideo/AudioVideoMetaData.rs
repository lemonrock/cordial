// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Default, Debug, Clone)]
pub(crate) struct AudioVideoMetaData
{
	#[serde(default)] pub(crate) load: AudioVideoLoad,
	#[serde(default)] pub(crate) initially_muted: bool,
	#[serde(default)] pub(crate) loops: bool,
	#[serde(default)] pub(crate) placeholder: ResourceUrl,
	#[serde(default)] pub(crate) starts_at_seconds_inclusive: u32,
	#[serde(default)] pub(crate) ends_at_seconds_exclusive: Option<u32>,
	#[serde(default)] pub(crate) tracks: Vec<AudioVideoTrack>,
	#[serde(default = "AudioVideoMetaData::show_controls_default")] pub(crate) show_controls: bool,
	#[serde(default)] pub(crate) disabled_controls: BTreeSet<AudioVideoDisabledControl>,
	
	// Used by amp-video, Google Video Site Map, ?twitter player card? (if we decide to)
	#[serde(default)] pub(crate) abstracts: HashMap<Iso639Dash1Alpha2Language, Rc<AudioVideoAbstract>>,
	
	// Used by amp-video, amp-audio
	#[serde(default)] pub(crate) artist: Option<String>,
	#[serde(default)] pub(crate) album: Option<String>,
	#[serde(default)] pub(crate) artwork: Option<ResourceUrl>,
	
	// Used by site map (and some by mRSS: https://developers.google.com/webmasters/videosearch/markups)
	#[serde(default)] pub(crate) site_map_category: Option<String>,
	#[serde(default)] pub(crate) site_map_tags: ArrayVec<[String; 32]>,
	#[serde(default)] pub(crate) expiration_date: Option<DateTime<Utc>>,
	#[serde(default)] pub(crate) publication_date: Option<DateTime<Utc>>,
	#[serde(default)] pub(crate) rating: Option<AudioVideoStarRating>,
	#[serde(default)] pub(crate) views: Option<u64>,
	#[serde(default)] pub(crate) site_map_explicit: bool,
	#[serde(default)] pub(crate) country_restrictions: AudioVideoCountryRestriction,
	#[serde(default)] pub(crate) platform_restrictions: AudioVideoPlatformRestriction,
	#[serde(default)] pub(crate) gallery: Option<ResourceUrl>,
	#[serde(default)] pub(crate) requires_subscription: bool,
	#[serde(default)] pub(crate) uploader: Option<Person>,
	
	// Used by Podcasts
	#[serde(default)] pub(crate) googleplay_author: Option<FullName>,
	#[serde(default)] pub(crate) itunes_author: Option<FullName>,
	#[serde(default)] pub(crate) googleplay_block: Option<bool>,
	#[serde(default)] pub(crate) itunes_block: bool,
	#[serde(default)] pub(crate) season_number: NonZeroNumber,
	#[serde(default)] pub(crate) episode_number: NonZeroNumber,
	#[serde(default)] pub(crate) episode_order: Option<NonZeroNumber>,
	#[serde(default)] pub(crate) episode_type: ITunesEpisodeType,
	#[serde(default)] pub(crate) googleplay_explicit: Option<bool>,
	#[serde(default)] pub(crate) itunes_explicit: bool,
	#[serde(default)] pub(crate) itunes_artwork: Option<ResourceUrl>,
	#[serde(default)] pub(crate) close_captioned: bool,
	
	// Used by mRSS
	#[serde(default)] pub(crate) licence: Option<ResourceUrl>,
	
	#[serde(default, skip_deserializing)] pub(crate) orderedMapOfWebVttUrls: RefCell<OrderMap<(AudioVideoTrackKind, Iso639Dash1Alpha2Language), Url>>,
}

impl AudioVideoMetaData
{
	#[inline(always)]
	pub(crate) fn createAudioMp4(&self, durationInSeconds: u64, mp4Url: Url, headerGenerator: &mut HeaderGenerator, mp4Body: Vec<u8>, result: &mut Vec<PipelineResource>, max_age_in_seconds: u32) -> Result<(), CordialError>
	{
		const Incompressible: bool = false;
		
		let mp4Headers = headerGenerator.generateHeadersForAsset(Incompressible, max_age_in_seconds, self.isDownloadable(), &mp4Url)?;
		let urlDataDetails = Rc::new(UrlDataDetails::audio(&mp4Body, durationInSeconds));
		let mp4Tags = hashmap!
		{
			ResourceTag::audio_mp4 => urlDataDetails.clone(),
			ResourceTag::default => urlDataDetails,
		};
		result.push((mp4Url, mp4Tags, StatusCode::Ok, ContentType(mimeType(Self::AudioMp4TwitterMimeType)), mp4Headers, mp4Body, None, Incompressible));
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn createVideoMp4(&self, width: u16, height: u16, durationInSeconds: u64, mp4Url: Url, headerGenerator: &mut HeaderGenerator, mp4Body: Vec<u8>, result: &mut Vec<PipelineResource>, max_age_in_seconds: u32) -> Result<(), CordialError>
	{
		const Incompressible: bool = false;
		
		let mp4Headers = headerGenerator.generateHeadersForAsset(Incompressible, max_age_in_seconds, self.isDownloadable(), &mp4Url)?;
		let urlDataDetails = Rc::new(UrlDataDetails::video(&mp4Body, width, height, durationInSeconds));
		let mp4Tags = hashmap!
		{
			ResourceTag::video_mp4 => urlDataDetails.clone(),
			ResourceTag::default => urlDataDetails,
		};
		result.push((mp4Url, mp4Tags, StatusCode::Ok, ContentType(mimeType(Self::VideoMp4TwitterMimeType)), mp4Headers, mp4Body, None, Incompressible));
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn createWebm(&self, width: u16, height: u16, durationInSeconds: u64, webmUrl: Url, headerGenerator: &mut HeaderGenerator, inputContentFilePath: &Path, result: &mut Vec<PipelineResource>, max_age_in_seconds: u32) -> Result<(), CordialError>
	{
		const Incompressible: bool = false;
		
		let webmInputContentFilePath = inputContentFilePath.with_extension("webm");
		let webmBody = webmInputContentFilePath.fileContentsAsBytes().context(webmInputContentFilePath)?;
		let webmHeaders = headerGenerator.generateHeadersForAsset(Incompressible, max_age_in_seconds, self.isDownloadable(), &webmUrl)?;
		let webmTags = hashmap!
		{
			ResourceTag::video_webm => Rc::new(UrlDataDetails::video(&webmBody, width, height, durationInSeconds))
		};
		result.push((webmUrl, webmTags, StatusCode::Ok, ContentType(mimeType(Self::WebMVp8MimeType)), webmHeaders, webmBody, None, Incompressible));
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn createIFramePlayer(resourceUrl: &ResourceUrl, audioOrVideoNode: UnattachedNode, width: u16, languageData: &LanguageData, headerGenerator: &mut HeaderGenerator, result: &mut Vec<PipelineResource>, max_age_in_seconds: u32) -> Result<(), CordialError>
	{
		#[inline(always)]
		fn iFramePlayerHtmlBody(audioVideoNode: UnattachedNode, width: u16) -> Vec<u8>
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
				.with_child_element(audioVideoNode)
			);
			
			htmlNode.to_html5_document(true).into_bytes()
		}
		const Compressible: bool = true;
		
		let iFramePlayerUrl = Self::iFramePlayerUrl(resourceUrl, languageData)?;
		let iFramePlayerBody = iFramePlayerHtmlBody(audioOrVideoNode, width);
		let iFramePlayerHeaders = headerGenerator.generateHeadersForAsset(Compressible, max_age_in_seconds, false, &iFramePlayerUrl)?;
		let iFramePlayerTags = hashmap!
		{
			ResourceTag::audio_video_iframe_player => Rc::new(UrlDataDetails::generic(&iFramePlayerBody))
		};
		result.push((iFramePlayerUrl, iFramePlayerTags, StatusCode::Ok, ContentType::html(), iFramePlayerHeaders, iFramePlayerBody, None, Compressible));
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn createWebVttTracks(&self, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, configuration: &Configuration, headerGenerator: &mut HeaderGenerator, result: &mut Vec<PipelineResource>, max_age_in_seconds: u32) -> Result<(), CordialError>
	{
		for track in self.tracks.iter()
		{
			const CanBeCompressed: bool = true;
			
			configuration.visitLanguagesWithPrimaryFirst(|languageData, _isPrimaryLanguage|
			{
				if let Some((webVttBody, webVttUrl)) = track.bodyAndUrl(languageData, inputContentFilePath, resourceUrl)?
				{
					self.orderedMapOfWebVttUrls.borrow_mut().insert((track.kind, languageData.iso639Dash1Alpha2Language), webVttUrl.clone());
					
					let webVttHeaders = headerGenerator.generateHeadersForAsset(CanBeCompressed, max_age_in_seconds, self.isDownloadable(), &webVttUrl)?;
					result.push((webVttUrl, hashmap! { ResourceTag::audio_video_track(track.kind, languageData.iso639Dash1Alpha2Language) => Rc::new(UrlDataDetails::generic(&webVttBody)) }, StatusCode::Ok, ContentType(mimeType("text/vtt")), webVttHeaders, webVttBody, None, CanBeCompressed));
				}
				
				Ok(())
			})?;
		}
		Ok(())
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn createAmpAudioNode(&self, resources: &Resources, configuration: &Configuration, languageData: &LanguageData, mp4Url: &Url, durationInSeconds: u64, volume: AudioVolume, placeHolderWidth: u16, placeHolderHeight: u16) -> Result<UnattachedNode, CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		let title = self.title(configuration, iso639Dash1Alpha2Language)?;
		
		let mut ampAudioNode =
			"amp-audio"
			.with_attributes
			(
				vec!
				[
					"title".str_attribute(title),
					AmpLayout::fixed_height.toAttribute(),
				]
			)
		;
		
		ampAudioNode = self.load.addToAudioOrVideoNode(ampAudioNode, durationInSeconds);
		
		if self.initially_muted || self.load == AudioVideoLoad::auto_play
		{
			ampAudioNode = ampAudioNode.with_empty_attribute("muted");
		}
		
		if self.loops
		{
			ampAudioNode = ampAudioNode.with_empty_attribute("loop");
		}
		
		if self.show_controls
		{
			ampAudioNode = ampAudioNode.with_empty_attribute("controls");
			
			if !self.disabled_controls.is_empty()
			{
				ampAudioNode = ampAudioNode.with_attribute("controlslist".space_separated_attribute(self.disabled_controls.iter().map(|disabled_control| disabled_control.deref())));
			}
		}
		
		if self.disabled_controls.contains(&AudioVideoDisabledControl::noremoteplayback)
		{
			ampAudioNode = ampAudioNode.with_empty_attribute("disableremoteplayback");
		}
		
		if let Some(ref artist) = self.artist
		{
			ampAudioNode = ampAudioNode.with_attribute("artist".str_attribute(artist));
		}
		
		if let Some(ref album) = self.album
		{
			ampAudioNode = ampAudioNode.with_attribute("album".str_attribute(album));
		}
		
		if let Some(ref artwork) = self.artwork
		{
			// 256x256 or 512x512? Whilst artwork is an array, it's not clear if amp-audio supports it.
			const ArtworkWidth: u32 = 512;
			const ArtworkHeight: u32 = 512;
			
			let artworkUrlData = ResourceReference
			{
				resource: artwork.clone(),
				tag: ResourceTag::width_height_image(ArtworkWidth, ArtworkHeight)
			}.urlDataMandatory(resources, configuration.fallbackIso639Dash1Alpha2Language(), Some(iso639Dash1Alpha2Language))?;
			artworkUrlData.validateIsPng()?;
			
			ampAudioNode = ampAudioNode.with_attribute("artwork".str_attribute(artworkUrlData.url_str()));
		}
		
		let placeHolderUrlData = self.placeHolderUrlData(resources, configuration, iso639Dash1Alpha2Language, placeHolderWidth, placeHolderHeight)?;
		
		ampAudioNode = ampAudioNode
		.with_child_element
		(
			"noscript"
			.with_child_element(self.createAudioNode(configuration, languageData, mp4Url, durationInSeconds, volume)?)
		)
		.with_child_element(self.createAmpImgPlaceHolderUrlData(placeHolderUrlData.as_ref(), resources)?)
		.with_child_element
		(
			"div"
			.with_empty_attribute("fallback")
			.with_child_text(languageData.requiredTranslation(RequiredTranslation::missing_audio_fallback)?.as_str())
		);
		
		self.addSources(ampAudioNode, mp4Url, None, Self::AudioMp4TwitterMimeType)
	}
	
	#[inline(always)]
	pub(crate) fn createAudioNode(&self, configuration: &Configuration, languageData: &LanguageData, mp4Url: &Url, durationInSeconds: u64, volume: AudioVolume) -> Result<UnattachedNode, CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		let title = self.title(configuration, iso639Dash1Alpha2Language)?;
		
		let mut audioNode =
			"audio"
			.with_attributes
			(
				vec!
				[
					"title".str_attribute(title),
				]
			)
		;
		
		audioNode = volume.writeToAudioNode(audioNode);
		
		audioNode = self.load.addToAudioOrVideoNode(audioNode, durationInSeconds);
		
		if self.initially_muted || self.load == AudioVideoLoad::auto_play
		{
			audioNode = audioNode.with_empty_attribute("muted");
		}
		
		if self.loops
		{
			audioNode = audioNode.with_empty_attribute("loop");
		}
		
		if self.show_controls
		{
			audioNode = audioNode.with_empty_attribute("controls");
			
			if !self.disabled_controls.is_empty()
			{
				audioNode = audioNode.with_attribute("controlsList".space_separated_attribute(self.disabled_controls.iter().map(|disabled_control| disabled_control.deref())));
			}
		}
		
		audioNode = self.addSources(audioNode, mp4Url, None, Self::AudioMp4TwitterMimeType)?;
		
		let translation = languageData.requiredTranslation(RequiredTranslation::your_browser_does_not_support_audio)?;
		audioNode = audioNode.with_child_text(translation.deref().as_str());
		
		Ok(audioNode)
	}
	
	#[inline(always)]
	fn createAmpImgPlaceHolderUrlData(&self, placeHolderUrlData: &UrlData, resources: &Resources) -> Result<UnattachedNode, CordialError>
	{
		let mut attributes = vec!
		[
			"placeholder".empty_attribute(),
			"src".str_attribute(placeHolderUrlData.url_str()),
			AmpLayout::fill.toAttribute(),
		];
		
		// Add "srcset" attribute
		let resource = self.placeholder.resourceMandatory(resources)?;
		let processedImageSourceSet = resource.processedImageSourceSet()?;
		ProcessedImageSourceSet::addToImgAttributes(processedImageSourceSet, &mut attributes)?;
		
		Ok("amp-img".with_attributes(attributes))
	}
	
	#[inline(always)]
	fn placeHolderUrlData(&self, resources: &Resources, configuration: &Configuration, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, width: u16, height: u16) -> Result<Rc<UrlData>, CordialError>
	{
		let placeHolderUrlData = ResourceReference
		{
			resource: self.placeholder.clone(),
			tag: ResourceTag::width_height_image(width as u32, height as u32)
		}.urlDataMandatory(resources, configuration.fallbackIso639Dash1Alpha2Language(), Some(iso639Dash1Alpha2Language))?;
		placeHolderUrlData.validateIsPng()?;
		Ok(placeHolderUrlData)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn createAmpVideoNode(&self, resources: &Resources, configuration: &Configuration, languageData: &LanguageData, width: u16, height: u16, mp4Url: &Url, webmUrl: &Url, durationInSeconds: u64, plays_inline: bool) -> Result<UnattachedNode, CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		let title = self.title(configuration, iso639Dash1Alpha2Language)?;
		
		let placeHolderUrlData = self.placeHolderUrlData(resources, configuration, iso639Dash1Alpha2Language, width, height)?;
		
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
					AmpLayout::responsive.toAttribute(),
				]
			)
		;
		
		ampVideoNode = self.load.addToAudioOrVideoNode(ampVideoNode, durationInSeconds);
		
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
		
		if let Some(ref artist) = self.artist
		{
			ampVideoNode = ampVideoNode.with_attribute("artist".str_attribute(artist));
		}
		
		if let Some(ref album) = self.album
		{
			ampVideoNode = ampVideoNode.with_attribute("album".str_attribute(album));
		}
		
		if let Some(ref artwork) = self.artwork
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
				.with_child_element(self.createVideoNode(resources, configuration, languageData, width, height, mp4Url, webmUrl, durationInSeconds, plays_inline)?)
			)
			.with_child_element(self.createAmpImgPlaceHolderUrlData(&placeHolderUrlData, resources)?)
			.with_child_element
			(
				"div"
				.with_empty_attribute("fallback")
				.with_child_text(languageData.requiredTranslation(RequiredTranslation::missing_video_fallback)?.as_str())
			);
		
		self.addVideoSourcesAndTracks(ampVideoNode, mp4Url, webmUrl, iso639Dash1Alpha2Language, configuration)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn createVideoNode(&self, resources: &Resources, configuration: &Configuration, languageData: &LanguageData, width: u16, height: u16, mp4Url: &Url, webmUrl: &Url, durationInSeconds: u64, plays_inline: bool) -> Result<UnattachedNode, CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		let title = self.title(configuration, iso639Dash1Alpha2Language)?;
		
		let placeHolderUrlData = self.placeHolderUrlData(resources, configuration, iso639Dash1Alpha2Language, width, height)?;
		
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
		
		videoNode = self.load.addToAudioOrVideoNode(videoNode, durationInSeconds);
		
		// Twitter Player Card Rules: Default to ‘sound off’ for videos that automatically play content
		if self.initially_muted || self.load == AudioVideoLoad::auto_play
		{
			videoNode = videoNode.with_empty_attribute("muted");
		}
		
		if plays_inline
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
		
		videoNode = self.addVideoSourcesAndTracks(videoNode, mp4Url, webmUrl, iso639Dash1Alpha2Language, configuration)?;
		
		let translation = languageData.requiredTranslation(RequiredTranslation::your_browser_does_not_support_video)?;
		videoNode = videoNode.with_child_text(translation.deref().as_str());
		
		Ok(videoNode)
	}
	
	fn addVideoSourcesAndTracks(&self, videoNode: UnattachedNode, mp4Url: &Url, webmUrl: &Url, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, configuration: &Configuration) -> Result<UnattachedNode, CordialError>
	{
		let videoNode = self.addSources(videoNode, mp4Url, Some(webmUrl), Self::VideoMp4TwitterMimeType)?;
		self.addTracks(videoNode, iso639Dash1Alpha2Language, configuration)
	}
	
	fn addSources(&self, mut videoNode: UnattachedNode, mp4Url: &Url, webmUrl: Option<&Url>, mp4Type: &str) -> Result<UnattachedNode, CordialError>
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
		
		if let Some(webmUrl) = webmUrl
		{
			videoNode = videoNode.with_child_element
			(
				"source"
				.with_type_attribute(Self::WebMVp8MimeType)
				.with_attribute("src".string_attribute(format!("{}{}", webmUrl.as_ref(), &mediaTimeFragment)))
			);
		}
		
		videoNode = videoNode.with_child_element
		(
			"source"
			.with_type_attribute(mp4Type)
			.with_attribute("src".string_attribute(format!("{}{}", mp4Url.as_ref(), &mediaTimeFragment)))
		);
		
		Ok(videoNode)
	}
	
	fn addTracks(&self, videoNode: UnattachedNode, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, configuration: &Configuration) -> Result<UnattachedNode, CordialError>
	{
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
	fn title(&self, configuration: &Configuration, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&str, CordialError>
	{
		Ok(&self.audioVideoAbstract(configuration.fallbackIso639Dash1Alpha2Language(), iso639Dash1Alpha2Language)?.title)
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
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn iFramePlayerUrl(resourceUrl: &ResourceUrl, languageData: &LanguageData) -> Result<Url, CordialError>
	{
		resourceUrl.replaceFileNameExtension(".iframe-player.html").url(languageData)
	}
	
	// See http://www.leanbackplayer.com/test/h5mt.html for most variants
	
	//noinspection SpellCheckingInspection
	pub(crate) const AudioMp4TwitterMimeType: &'static str = "audio/mp4;codecs=\"mp4a.40.2\"";
	
	//noinspection SpellCheckingInspection
	pub(crate) const VideoMp4TwitterMimeType: &'static str = "video/mp4;codecs=\"avc1.42E01E,mp4a.40.2\"";
	
	//noinspection SpellCheckingInspection
	const WebMVp8MimeType: &'static str = "video/webm;codecs=\"vp8,vorbis\"";
	
	#[inline(always)]
	pub(crate) fn audioVideoAbstract(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&AudioVideoAbstract, CordialError>
	{
		if let Some(audioVideoAbstract) = self.abstracts.get(&iso639Dash1Alpha2Language)
		{
			Ok(audioVideoAbstract)
		}
		else if let Some(audioVideoAbstract) = self.abstracts.get(&fallbackIso639Dash1Alpha2Language)
		{
			Ok(audioVideoAbstract)
		}
		else
		{
			return Err(CordialError::Configuration("no AudioVideoAbstract for primary or fallback language".to_owned()))
		}
	}
	
	#[inline(always)]
	pub(crate) fn writeSiteMapXml<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, mediaUrl: &Url, iFrameUrl: &Url, durationInSeconds: Option<u64>) -> Result<(), CordialError>
	{
		let thumbnailResource = self.placeholder.resourceMandatory(resources)?;
		let thumbnailUrlData = thumbnailResource.findGoogleVideoSiteMapImageThumbnail(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "thumbnail_loc", thumbnailUrlData.url_str())?;
		
		self.audioVideoAbstract(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?.writeSiteMapXml(eventWriter, namespace, emptyAttributes)?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "content_loc", mediaUrl.as_ref())?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "player_loc", iFrameUrl.as_ref())?;
		
		let live = if let Some(durationInSeconds) = durationInSeconds
		{
			const GoogleMaximumDurationOfEightHoursInSeconds: u64 = 28_800;
			if durationInSeconds > GoogleMaximumDurationOfEightHoursInSeconds
			{
				return Err(CordialError::Configuration("Audio or video in site maps can not exceed eight hours duration".to_owned()));
			}
			eventWriter.writePrefixedTextElementU64(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "duration", durationInSeconds)?;
			Self::BooleanNo
		}
		else
		{
			Self::BooleanYes
		};
		
		if let Some(expirationDate) = self.expiration_date
		{
			eventWriter.writePrefixedTextElementRfc3339(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "expiration_date", expirationDate)?;
		}
		
		if let Some(videoStarRating) = self.rating
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "rating", &videoStarRating.toGoogleSiteMapString())?;
		}
		
		if let Some(viewCount) = self.views
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "view_count", &format!("{}", viewCount))?;
		}
		
		if let Some(publicationDate) = self.publication_date
		{
			eventWriter.writePrefixedTextElementRfc3339(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "publication_date", publicationDate)?;
		}
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "family_friendly", Self::booleanYesOrNo(self.site_map_explicit))?;
		
		self.writeXmlForCanonicalizedTagString(eventWriter, namespace, emptyAttributes)?;
		
		self.writeXmlForCategory(eventWriter, namespace, emptyAttributes)?;
		
		self.country_restrictions.writeSiteMapXmlForRestriction(eventWriter, namespace)?;
		
		if let Some(ref gallery) = self.gallery
		{
			let (url, title) = ResourceReference
			{
				resource: gallery.clone(),
				tag: ResourceTag::default,
			}.urlAndAnchorTitleAttribute(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			
			eventWriter.writePrefixedTextElement(namespace, &[ "title".xml_str_attribute(title.as_str()) ], SiteMapWebPageAudioVideo::VideoNamespacePrefix, "gallery_loc", url.as_str())?;
		}
		
		// Unimplemented: video:price (can be supported with a BTreeSet of (currency, type, resolution) tuples
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "requires_subscription", Self::booleanYesOrNo(self.requires_subscription))?;
		
		#[inline(always)]
		fn writeUploader<'a, W: Write>(eventWriter: &mut EventWriter<W>, namespace: &Namespace, uploaderName: &FullName, attributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
		{
			eventWriter.writePrefixedTextElement(namespace, &attributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "uploader", uploaderName)
		}
		
		if let Some(ref uploader) = self.uploader
		{
			let uploaderName = &uploader.full_name;
			
			if let Some(ref resourceUrl) = uploader.url
			{
				let url = ResourceReference
				{
					resource: resourceUrl.clone(),
					tag: ResourceTag::default,
				}.urlMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
				
				writeUploader(eventWriter, namespace, uploaderName, &["info".xml_url_attribute(&url)])?;
			}
			else
			{
				writeUploader(eventWriter, namespace, uploaderName, &emptyAttributes)?;
			}
		}
		
		self.platform_restrictions.writeSiteMapXmlForRestriction(eventWriter, namespace)?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "live", live)
	}
	
	const BooleanYes: &'static str = "yes";
	
	const BooleanNo: &'static str = "no";
	
	#[inline(always)]
	fn booleanYesOrNo(boolean: bool) -> &'static str
	{
		if boolean
		{
			Self::BooleanYes
		}
		else
		{
			Self::BooleanNo
		}
	}
	
	#[inline(always)]
	fn writeXmlForCanonicalizedTagString<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		let mut canonicalizedOrderedTags = OrderMap::with_capacity(self.site_map_tags.len());
		for toBeCanonicalizedTag in self.site_map_tags.iter()
		{
			let lowerCased = toBeCanonicalizedTag.to_lowercase();
			if !canonicalizedOrderedTags.contains_key(&lowerCased)
			{
				canonicalizedOrderedTags.insert(lowerCased, ());
			}
		}
		for canonicalizedSortedTag in canonicalizedOrderedTags.keys()
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "tag", canonicalizedSortedTag)?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn writeXmlForCategory<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		if let Some(ref category) = self.site_map_category
		{
			if category.chars().count() > 256
			{
				return Err(CordialError::Configuration("Audio / Video site map category can not exceed 256 characters".to_owned()));
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "category", category)
		}
		else
		{
			Ok(())
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn writePodcastRssXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, podcastUrlData: &UrlData) -> Result<(), CordialError>
	{
		let audioVideoAbstract = self.audioVideoAbstract(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		audioVideoAbstract.writePodcastRssXml(eventWriter, namespace, emptyAttributes)?;
		
		let length = "length".xml_u64_attribute(podcastUrlData.size());
		let enclosureAttributes =
		[
			"url".xml_str_attribute(podcastUrlData.url_str()),
			length.borrow(),
			"type".xml_str_attribute(podcastUrlData.mimeType().as_ref()),
		];
		eventWriter.writeEmptyElement(namespace, &enclosureAttributes, "enclosure".xml_local_name())?;
		
		if let Some(ref googlePlayAuthor) = self.googleplay_author
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::GooglePlayNamespacePrefix, "author", googlePlayAuthor)?;
		}
		
		if let Some(ref iTunesAuthor) = self.itunes_author
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "author", iTunesAuthor)?;
		}
		
		if let Some(googlePlayBlock) = self.googleplay_block
		{
			if googlePlayBlock
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::GooglePlayNamespacePrefix, "block", "yes")?;
			}
		}
		
		if self.itunes_block
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "block", ITunesBooleanYes)?;
		}
		
		let durationInSeconds = podcastUrlData.durationInSeconds()?;
		const SecondsInAMinute: u64 = 60;
		const MinutesInAnHour: u64 = 60;
		const SecondsInAnHour: u64 = SecondsInAMinute * MinutesInAnHour;
		let hours = durationInSeconds / SecondsInAnHour;
		let remainingSeconds = durationInSeconds % SecondsInAnHour;
		let minutes = remainingSeconds / SecondsInAMinute;
		let seconds = remainingSeconds % SecondsInAMinute;
		let formattedDuration = if hours > 0
		{
			format!("{}:{:02}:{:02}", hours, minutes, seconds)
		}
		else if minutes > 9
		{
			format!("{:02}:{:02}", minutes, seconds)
		}
		else if minutes > 0
		{
			format!("{}:{:02}", minutes, seconds)
		}
		else
		{
			format!("{}", seconds)
		};
		eventWriter.writePrefixedTextElementString(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "duration", formattedDuration)?;
		
		eventWriter.writePrefixedTextElementU16(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "episode", self.episode_number.0)?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "episodeType", self.episode_type.to_str())?;
		
		if let Some(googlePlayExplicit) = self.googleplay_explicit
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::GooglePlayNamespacePrefix, "explicit", googlePlayExplicitness(googlePlayExplicit))?;
		}
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "explicit", iTunesExplicitness(self.itunes_explicit))?;
		
		if let Some(ref iTunesArtwork) = self.itunes_artwork
		{
			let resource = iTunesArtwork.resourceMandatory(resources)?;
			let urlData = resource.findITunesRssArtwork(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "image", urlData.url_str())?;
		}
		
		if self.close_captioned
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "isCloseCaptioned", ITunesBooleanYes)?;
		}
		
		if let Some(episodeOrder) = self.episode_order
		{
			eventWriter.writePrefixedTextElementU16(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "order", episodeOrder.0)?;
		}
		
		eventWriter.writePrefixedTextElementU16(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "season", self.season_number.0)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn writeVideoMRssXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, podcastUrlData: &UrlData, iFrameUrl: &Url) -> Result<(), CordialError>
	{
		// Bing also uses xmlns:bing="http://bing.com/schema/media/" which is different... oh great. https://blogs.bing.com/webmaster/2012/04/24/video-killed-the-radio-star-but-we-still-want-your-videos/
		
		// Baidu: http://video.baidu.com/videoop.html
		
		// expression determines if the object is a sample or the full version of the object, or even if it is a continuous stream (sample | full | nonstop). Default value is "full". It is an optional attribute.
		// mRSS <media:content>; used by MailChimp and Bing, for instance, and also used somewhat by Google: https://developers.google.com/webmasters/videosearch/markups
		
		const IsLiveVideoBecauseThereIsNoDuration: bool = false;
		
		// media:content
		{
			let optionalVideoWidthHeight = podcastUrlData.optionalVideoWidthHeight();
			
			let medium = if optionalVideoWidthHeight.is_some()
			{
				"video"
			}
			else
			{
				"image"
			};
			
			let widthAndHeightAttribute = if let Some((width, height)) = optionalVideoWidthHeight
			{
				Some(("width".xml_u16_attribute(width), "height".xml_u16_attribute(height)))
			}
			else
			{
				None
			};
			
			let fileSizeAttribute = "fileSize".xml_u64_attribute(podcastUrlData.size());
			let durationAttribute = "duration".xml_u64_attribute(podcastUrlData.durationInSeconds()?);
			let mut contentAttributes = vec!
			[
				"url".xml_url_from_UrlData_attribute(podcastUrlData),
				"medium".xml_str_attribute(medium),
				fileSizeAttribute.borrow(),
				"type".xml_str_attribute(podcastUrlData.mimeType().as_ref()),
				// need to know if underlying resource is language sensitive. Alternatively use 'x-default'.
				//"lang".xml_str_attribute(iso639Dash1Alpha2Language.to_iso_639_1_alpha_2_language_code()),
			];
			
			if IsLiveVideoBecauseThereIsNoDuration
			{
				contentAttributes.push("expresision".xml_str_attribute("nonstop"))
			}
			else
			{
				contentAttributes.push("expresision".xml_str_attribute(self.episode_type.toMediaRssStr()));
				contentAttributes.push(durationAttribute.borrow());
			}
			
			if let Some((ref widthAttribute, ref heightAttribute)) = widthAndHeightAttribute
			{
				contentAttributes.push(widthAttribute.borrow());
				contentAttributes.push(heightAttribute.borrow());
			}
			
			self.writeVideoMRssMediaContent(eventWriter, namespace, emptyAttributes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, iFrameUrl, contentAttributes)?;
		}
		
		self.country_restrictions.writeMRssXmlForRestriction(eventWriter, namespace)?;
		
		let validity = if let Some(publication_date) = self.publication_date
		{
			if let Some(expiration_date) = self.expiration_date
			{
				Some(format!("start={}; end={}; scheme=W3C-DTF", publication_date.to_rfc3339(), expiration_date.to_rfc3339()))
			}
			else
			{
				Some(format!("start={}; scheme=W3C-DTF", publication_date.to_rfc3339()))
			}
		}
		else if let Some(expiration_date) = self.expiration_date
		{
			Some(format!("end={}; scheme=W3C-DTF", expiration_date.to_rfc3339()))
		}
		else
		{
			None
		};
		
		if let Some(validity) = validity
		{
			eventWriter.writePrefixedTextElementString(namespace, emptyAttributes, RssChannel::DcTermsNamespacePrefix, "valid", validity)?;
		}
		
		if IsLiveVideoBecauseThereIsNoDuration
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::DcTermsNamespacePrefix, "type", "live-video")?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn writeVideoMRssMediaContent<'a: 'b, 'b, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iFrameUrl: &Url, contentAttributes: Vec<XmlAttribute<'b>>) -> Result<(), CordialError>
	{
		let audioVideoAbstract = self.audioVideoAbstract(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		
		eventWriter.writeWithinElement(RssChannel::MediaNamespacePrefix.prefixes_xml_name("content"), &namespace, &contentAttributes, |eventWriter|
		{
			let thumbnailAttributes =
			[
				"url".xml_str_attribute(iFrameUrl.as_ref()),
			];
			eventWriter.writeEmptyElement(namespace, &thumbnailAttributes, RssChannel::MediaNamespacePrefix.prefixes_xml_name("player"))?;
			
			eventWriter.writeTextElement(namespace, &["type".xml_str_attribute("plain")], RssChannel::MediaNamespacePrefix.prefixes_xml_name("title"), &audioVideoAbstract.title)?;
			
			eventWriter.writeTextElement(namespace, &["type".xml_str_attribute("plain")], RssChannel::MediaNamespacePrefix.prefixes_xml_name("description"), &audioVideoAbstract.site_map_description)?;
			
			let thumbnailResource = self.placeholder.resourceMandatory(resources)?;
			let thumbnailUrlData = thumbnailResource.findGoogleVideoSiteMapImageThumbnail(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
			let (width, height) = thumbnailUrlData.dimensions()?;
			let widthAttribute = "width".xml_u32_attribute(width);
			let heightAttribute = "height".xml_u32_attribute(height);
			let thumbnailAttributes =
			[
				"url".xml_url_from_UrlData_attribute(thumbnailUrlData),
				widthAttribute.borrow(),
				heightAttribute.borrow(),
			];
			eventWriter.writeEmptyElement(namespace, &thumbnailAttributes, RssChannel::MediaNamespacePrefix.prefixes_xml_name("thumbnail"))?;
			
			// media:price is omitted except for requires_subscription
			if self.requires_subscription
			{
				eventWriter.writeEmptyElement(namespace, &["type".xml_str_attribute("subscription")], RssChannel::MediaNamespacePrefix.prefixes_xml_name("price"))?;
			}
			
			// Not part of Google mRSS
			if let Some(ref uploader) = self.uploader
			{
				eventWriter.writeTextElement(namespace, &emptyAttributes, RssChannel::MediaNamespacePrefix.prefixes_xml_name("credit"), &uploader.full_name)?;
			}
			
			// Not part of Google mRSS
			if let Some(ref licence) = self.licence
			{
				let (licenseUrl, licenseTitle) = ResourceReference
				{
					resource: licence.clone(),
					tag: ResourceTag::default,
				}.urlAndAnchorTitleAttribute(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
				
				let licenseUrl = licenseUrl.as_ref().as_str();
				eventWriter.writeTextElement(namespace, &["url".xml_str_attribute(licenseUrl)], RssChannel::MediaNamespacePrefix.prefixes_xml_name("copyright"), &licenseTitle)?;
				eventWriter.writeTextElement(namespace, &["type".xml_str_attribute("text/html"), "href".xml_str_attribute(licenseUrl)], RssChannel::MediaNamespacePrefix.prefixes_xml_name("license"), &licenseTitle)?;
			}
			
			Ok(())
		})
	}
	
	#[inline(always)]
	fn show_controls_default() -> bool
	{
		true
	}
}
