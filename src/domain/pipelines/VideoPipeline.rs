// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct VideoPipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default)] input_format: Option<VideoInputFormat>,
	
	#[serde(default)] pub(crate) metadata: Rc<AudioVideoMetaData>,
	#[serde(default)] pub(crate) plays_inline: bool,
	
	#[serde(default, skip_deserializing)] pub(crate) dimensions: Cell<(u16, u16)>,
	#[serde(default, skip_deserializing)] pub(crate) durationInSeconds: Cell<u64>,
	#[serde(default, skip_deserializing)] pub(crate) mp4Url: RefCell<Option<Url>>,
	#[serde(default, skip_deserializing)] pub(crate) webmUrl: RefCell<Option<Url>>,
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
			
			metadata: Default::default(),
			plays_inline: false,
			
			dimensions: Default::default(),
			durationInSeconds: Default::default(),
			mp4Url: Default::default(),
			webmUrl: Default::default(),
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
		
		let (width, height, durationInSeconds) = if isPrimaryLanguage
		{
			let mp4Url = self.mp4Url(resourceUrl, configuration)?;
			*self.mp4Url.borrow_mut() = Some(mp4Url);
			
			let webmUrl = self.webmUrl(resourceUrl, configuration)?;
			*self.webmUrl.borrow_mut() = Some(webmUrl);
			
			let mp4Body = inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?;
			
			let (width, height, durationInSeconds) = videoTrackDurationWidthAndHeight(&mp4Body)?;
			self.dimensions.set((width, height));
			self.durationInSeconds.set(durationInSeconds);
			
			self.metadata.createWebVttTracks(inputContentFilePath, resourceUrl, configuration, headerGenerator, &mut result, self.max_age_in_seconds)?;
			self.metadata.createVideoMp4(width, height, durationInSeconds, self.mp4Url.borrow().as_ref().unwrap().clone(), headerGenerator, mp4Body, &mut result, self.max_age_in_seconds)?;
			self.metadata.createWebm(width, height, durationInSeconds, self.mp4Url.borrow().as_ref().unwrap().clone(), headerGenerator, inputContentFilePath, &mut result, self.max_age_in_seconds)?;
			
			(width, height, durationInSeconds)
		}
		else
		{
			let (width, height) = self.dimensions.get();
			(width, height, self.durationInSeconds.get())
		};
		
		let videoNode =
		{
			let mp4UrlBorrow = self.mp4Url.borrow();
			let webmUrlBorrow = self.webmUrl.borrow();
			self.metadata.createVideoNode(resources, configuration, languageData, width, height, mp4UrlBorrow.as_ref().unwrap(), &webmUrlBorrow.as_ref().unwrap(), durationInSeconds, self.plays_inline)?
		};
		
		AudioVideoMetaData::createIFramePlayer(resourceUrl, videoNode, width, languageData, headerGenerator, &mut result, self.max_age_in_seconds)?;
		
		Ok(result)
	}
}

impl VideoPipeline
{
	#[inline(always)]
	pub(crate) fn siteMapWebPageVideo(&self, resourceUrl: &ResourceUrl, languageData: &LanguageData, configuration: &Configuration) -> Result<SiteMapWebPageAudioVideo, CordialError>
	{
		Ok
		(
			SiteMapWebPageAudioVideo
			{
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
	pub(crate) fn twitterContentType(&self) -> &str
	{
		AudioVideoMetaData::VideoMp4TwitterMimeType
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
	
	#[inline(always)]
	pub(crate) fn iFramePlayerUrl(&self, resourceUrl: &ResourceUrl, languageData: &LanguageData) -> Result<Url, CordialError>
	{
		AudioVideoMetaData::iFramePlayerUrl(resourceUrl, languageData)
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
			self.metadata.createAmpVideoNode(resources, configuration, languageData, width, height, &mp4UrlBorrow.as_ref().unwrap(), &webmUrlBorrow.as_ref().unwrap(), durationInSeconds, self.plays_inline)
		}
		else
		{
			self.metadata.createVideoNode(resources, configuration, languageData, width, height, &mp4UrlBorrow.as_ref().unwrap(), &webmUrlBorrow.as_ref().unwrap(), durationInSeconds, self.plays_inline)
		}
	}
}
