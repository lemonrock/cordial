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
	#[serde(default)] pub(crate) placeholder: Option<ResourceUrl>,
	#[serde(default)] pub(crate) starts_at_seconds_inclusive: u32,
	#[serde(default)] pub(crate) ends_at_seconds_exclusive: Option<u32>,
	#[serde(default)] pub(crate) tracks: Vec<AudioVideoTrack>,
	#[serde(default = "VideoPipeline::show_controls_default")] pub(crate) show_controls: bool,
	#[serde(default)] pub(crate) disabled_controls: BTreeSet<AudioVideoDisabledControl>,
	
	// Used by amp-video, Google Video Site Map, ?twitter player card? (if we decide to)
	#[serde(default)] pub(crate) title: HashMap<Iso639Dash1Alpha2Language, Rc<String>>,
	
	#[serde(default)] pub(crate) artist: Option<String>,
	#[serde(default)] pub(crate) album: Option<String>,
	#[serde(default)] pub(crate) artwork: Option<ResourceUrl>,
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
			placeholder: None,
			starts_at_seconds_inclusive: 0,
			ends_at_seconds_exclusive: None,
			tracks: Default::default(),
			show_controls: Self::show_controls_default(),
			disabled_controls: Default::default(),
			title: Default::default(),
			
			artist: None,
			album: None,
			artwork: None,
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
		let mp4Body = inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?;
		
		let (width, height, optionalMp4VideoCodecString, optionalMp4AudioCodecString) = Self::mp4Metadata(&mp4Body, inputContentFilePath)?;
		let mp4Url = resourceUrl.replaceFileNameExtension(".mp4").url(languageData)?;
		let webmUrl = resourceUrl.replaceFileNameExtension(".webm").url(languageData)?;
		
		let mut result = Vec::new();
		
		let videoNode = self.createVideoNode(resourceUrl, resources, configuration, languageData, inputContentFilePath, width, height, &mp4Url, &webmUrl)?;
		
		let isPrimaryLanguage = configuration.fallbackIso639Dash1Alpha2Language() == languageData.iso639Dash1Alpha2Language;
		
		if isPrimaryLanguage
		{
			self.createWebVttTracks(inputContentFilePath, resourceUrl, configuration, headerGenerator, &mut result)?;
			self.createMp4(width, height, mp4Url, headerGenerator, mp4Body, &mut result)?;
			self.createWebm(width, height, webmUrl, headerGenerator, inputContentFilePath, &mut result)?;
		}
		
		self.createTwitterIFramePlayer(resourceUrl, videoNode, width, languageData, headerGenerator, &mut result)?;
		
		Ok(result)
		
		/*
			TODO: Schema.org for videos
				https://developers.google.com/webmasters/videosearch/schema
				- uses the embedded syntax, includes width & height
				- is this appropriate?
				- we can supply both a contentUrl and an embedUrl (the Twitter iframe)
				
				- need image ratios, ie 1x1, 4x3, 16x9
				
		
			TODO: Google Video sitemaps: https://developers.google.com/webmasters/videosearch/sitemaps
				- video 'thumbnails' (placeholders for us) must be 160x90 pixels and at most 1920x1080 pixels (1:1.77)
				- thumbnails are mandatory
			<video:video>
				<video:thumbnail_loc>https://url/image.png</>
				<video:title></>
				<video:content_loc>https://url/to/raw/mp4
				<video:duration>Duration_in_Seconds
				<video:expiration_date>
				<video:rating>
				<video:view_count>
				<video:publication_date>
				<video:family_friendly>
				<video:tag> 0 - 32 (incl); we should have translations; should match Facebook OpenGraph
				<video:category> Similar to article Section. Maximum 256 characters.
				<video:restriction> - blacklist or whitelist using ISO 3166 country codes; uses relationship attribute
				<video:gallery_loc>  - URL of a gallery page, with a title attribute
				HashSet of <video:price> (currency, type, resolution attributes)
				<video:requires_subscription>no</>
				<video:uploader info="https://url/of/uploader">Jo Bloggs</>
				<video:platform> with relationship attribute - specifies whitelist / blacklist for playing locations
					eg <video:platform relationship="allow">WEB TV</video:restriction>
					
				<video:live> - is this a live stream
			</video:video>
		*/
	}
}

impl VideoPipeline
{
	// See http://geekthis.net/post/c-get-mp4-duration/
	
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
					result.push((webVttUrl, hashmap! { default => Rc::new(UrlDataDetails::generic(&webVttBody)) }, StatusCode::Ok, ContentType(mimeType("text/vtt")), webVttHeaders, webVttBody, None, CanBeCompressed));
				}
				
				Ok(())
			})?;
		}
		Ok(())
	}
	
	#[inline(always)]
	fn createTwitterIFramePlayer(&self, resourceUrl: &ResourceUrl, videoNode: UnattachedNode, width: u32, languageData: &LanguageData, headerGenerator: &mut HeaderGenerator, result: &mut Vec<PipelineResource>) -> Result<(), CordialError>
	{
		const Compressible: bool = true;
		
		let twitterIFramePlayerUrl = resourceUrl.replaceFileNameExtension(".twitter-iframe-player.html").url(languageData)?;
		let twitterIFramePlayerBody = Self::twitterIFramePlayerHtmlBody(videoNode, width);
		let twitterIFramePlayerHeaders = headerGenerator.generateHeadersForAsset(Compressible, self.max_age_in_seconds, false, &twitterIFramePlayerUrl)?;
		let twitterIFramePlayerTags = hashmap!
		{
			default => Rc::new(UrlDataDetails::generic(&twitterIFramePlayerBody))
		};
		result.push((twitterIFramePlayerUrl, twitterIFramePlayerTags, StatusCode::Ok, ContentType(mimeType(Self::WebMVp8MimeType)), twitterIFramePlayerHeaders, twitterIFramePlayerBody, None, Compressible));
		Ok(())
	}
	
	#[inline(always)]
	fn createMp4(&self, width: u32, height: u32, mp4Url: Url, headerGenerator: &mut HeaderGenerator, mp4Body: Vec<u8>, result: &mut Vec<PipelineResource>) -> Result<(), CordialError>
	{
		const Incompressible: bool = false;
		
		let mp4Headers = headerGenerator.generateHeadersForAsset(Incompressible, self.max_age_in_seconds, self.isDownloadable(), &mp4Url)?;
		let mp4Tags = hashmap!
		{
			default => Rc::new(UrlDataDetails::video(&mp4Body, width, height))
		};
		result.push((mp4Url, mp4Tags, StatusCode::Ok, ContentType(mimeType(Self::Mp4TwitterMimeType)), mp4Headers, mp4Body, None, Incompressible));
		Ok(())
	}
	
	#[inline(always)]
	fn createWebm(&self, width: u32, height: u32, webmUrl: Url, headerGenerator: &mut HeaderGenerator, inputContentFilePath: &Path, result: &mut Vec<PipelineResource>) -> Result<(), CordialError>
	{
		const Incompressible: bool = false;
		
		let webmInputContentFilePath = inputContentFilePath.with_extension("webm");
		let webmBody = webmInputContentFilePath.fileContentsAsBytes().context(webmInputContentFilePath)?;
		let webmHeaders = headerGenerator.generateHeadersForAsset(Incompressible, self.max_age_in_seconds, self.isDownloadable(), &webmUrl)?;
		let webmTags = hashmap!
		{
			default => Rc::new(UrlDataDetails::video(&webmBody, width, height))
		};
		result.push((webmUrl, webmTags, StatusCode::Ok, ContentType(mimeType(Self::WebMVp8MimeType)), webmHeaders, webmBody, None, Incompressible));
		Ok(())
	}
	
	#[inline(always)]
	fn mp4Metadata(mp4Body: &[u8], mp4FilePath: &Path) -> Result<(u32, u32, Option<String>, Option<String>), CordialError>
	{
		use self::AudioVideoMetadata::*;
		use self::audio_video_metadata::enums::VideoType::*;
		use self::audio_video_metadata::enums::AudioType::*;
		
		let mp4Metadata = ::audio_video_metadata::get_format_from_slice(&mp4Body).context(mp4FilePath)?;
		
		let attributes = match mp4Metadata
		{
			Audio(_) => return Err(CordialError::Configuration(format!("{:?} is audio data not video data", mp4FilePath))),
			Video(videoMetadata) =>
			{
				match videoMetadata.format
				{
					MP4 => (),
					WebM => return Err(CordialError::Configuration(format!("{:?} is WebM; an MP4 video is needed for the primary resource", mp4FilePath))),
					_ => return Err(CordialError::Configuration(format!("{:?} is unsuitable video data ({:?})", mp4FilePath, &videoMetadata.format))),
				}
				
				let audioMetadata = videoMetadata.audio;
				match audioMetadata.format
				{
					MP3 => (),
					_ => return Err(CordialError::Configuration(format!("{:?} is unsuitable video data as it uses an unsuitable audio codec ({:?})", mp4FilePath, &videoMetadata.format))),
				}
				
				
				(videoMetadata.dimensions.width, videoMetadata.dimensions.height, videoMetadata.video, audioMetadata.audio)
			},
		};
		Ok(attributes)
	}
	
	#[inline(always)]
	fn twitterIFramePlayerHtmlBody(videoNode: UnattachedNode, width: u32) -> Vec<u8>
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
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn createVideoNode(&self, resourceUrl: &ResourceUrl, resources: &Resources, configuration: &Configuration, languageData: &LanguageData, inputContentFilePath: &Path, width: u32, height: u32, mp4Url: &Url, webmUrl: &Url) -> Result<UnattachedNode, CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		let title = match self.title.get(&iso639Dash1Alpha2Language)
		{
			None => return Err(CordialError::Configuration(format!("There is no title in {:?} for this video", iso639Dash1Alpha2Language))),
			Some(title) => title,
		};
		
		let mut videoNode =
			"video"
			.with_attributes
			(
				vec!
				[
					"width".string_attribute(format!("{}", width)),
					"height".string_attribute(format!("{}", height)),
					"title".str_attribute(title.as_str()),
				]
			)
		;
		
		videoNode = self.load.addToVideoNode(videoNode);
		
		if self.initially_muted
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
		
		if let Some(ref placeHolderUrl) = self.placeholder
		{
			let placeHolderUrlData = ResourceReference
			{
				resource: placeHolderUrl.clone(),
				tag: ResourceTag::width_height_image(width, height)
			}.urlDataMandatory(resources, configuration.fallbackIso639Dash1Alpha2Language(), Some(iso639Dash1Alpha2Language))?;
			placeHolderUrlData.validateIsPng()?;
			
			videoNode = videoNode.with_attribute("poster".str_attribute(placeHolderUrlData.url_str()))
		}
		
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
