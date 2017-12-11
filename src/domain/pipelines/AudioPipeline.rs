// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AudioPipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default)] input_format: Option<AudioInputFormat>,
	
	#[serde(default)] pub(crate) metadata: Rc<AudioVideoMetaData>,
	#[serde(default = "AudioPipeline::width_default")] pub(crate) width: u16,
	#[serde(default = "AudioPipeline::height_default")] pub(crate) height: u16,
	#[serde(default)] pub(crate) volume: AudioVolume,
	
	#[serde(default, skip_deserializing, skip_serializing)] pub(crate) durationInSeconds: Cell<u64>,
	#[serde(default, skip_deserializing, skip_serializing)] pub(crate) mp4Url: RefCell<Option<Url>>,
}

impl Default for AudioPipeline
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
			width: Self::width_default(),
			height: Self::height_default(),
			volume: Default::default(),
			
			durationInSeconds: Default::default(),
			mp4Url: Default::default(),
		}
	}
}

impl Pipeline for AudioPipeline
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
	fn execute(&self, _resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, _handlebars: &HandlebarsWrapper, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, configuration: &Configuration, _rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, _siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<PipelineResponse>, CordialError>
	{
		let isPrimaryLanguage = configuration.fallbackIso639Dash1Alpha2Language() == languageData.iso639Dash1Alpha2Language;
		
		let mut result = Vec::new();
		
		let durationInSeconds = if isPrimaryLanguage
		{
			let mp4Url = self.mp4Url(resourceUrl, configuration)?;
			*self.mp4Url.borrow_mut() = Some(mp4Url);
			
			let mp4Body = inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?;
			
			let durationInSeconds = onlyAudioTrackDuration(&mp4Body)?;
			self.durationInSeconds.set(durationInSeconds);
			
			self.metadata.createWebVttTracks(inputContentFilePath, resourceUrl, configuration, headerGenerator, &mut result, self.max_age_in_seconds)?;
			self.metadata.createAudioMp4(durationInSeconds, self.mp4Url.borrow().as_ref().unwrap().clone(), headerGenerator, mp4Body, &mut result, self.max_age_in_seconds)?;
			
			durationInSeconds
		}
		else
		{
			self.durationInSeconds.get()
		};
		
		let audioNode =
		{
			let mp4UrlBorrow = self.mp4Url.borrow();
			self.metadata.createAudioNode(configuration, languageData, mp4UrlBorrow.as_ref().unwrap(), durationInSeconds, self.volume)?
		};
		
		AudioVideoMetaData::createIFramePlayer(resourceUrl, audioNode, self.width, languageData, headerGenerator, &mut result, self.max_age_in_seconds)?;
		
		Ok(result)
	}
}

impl AudioPipeline
{
	#[inline(always)]
	pub(crate) fn siteMapWebPageAudio(&self, resourceUrl: &ResourceUrl, languageData: &LanguageData, configuration: &Configuration) -> Result<SiteMapWebPageAudioVideo, CordialError>
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
		(self.width, self.height)
	}
	
	#[inline(always)]
	pub(crate) fn twitterContentType(&self) -> ContentType
	{
		audioMp4TwitterContentType()
	}
	
	#[inline(always)]
	fn mp4Url(&self, resourceUrl: &ResourceUrl, configuration: &Configuration) -> Result<Url, CordialError>
	{
		resourceUrl.replaceFileNameExtension(".mp4").url(&configuration.primaryLanguageData()?)
	}
	
	#[inline(always)]
	pub(crate) fn iFramePlayerUrl(&self, resourceUrl: &ResourceUrl, languageData: &LanguageData) -> Result<Url, CordialError>
	{
		AudioVideoMetaData::iFramePlayerUrl(resourceUrl, languageData)
	}
	
	#[inline(always)]
	pub(crate) fn audioNode(&self, isForAmp: bool, resources: &Resources, configuration: &Configuration, languageData: &LanguageData) -> Result<UnattachedNode, CordialError>
	{
		let mp4UrlBorrow = self.mp4Url.borrow();
		
		let durationInSeconds = self.durationInSeconds.get();
		
		if isForAmp
		{
			self.metadata.createAmpAudioNode(resources, configuration, languageData, &mp4UrlBorrow.as_ref().unwrap(), durationInSeconds, self.volume, self.width, self.height)
		}
		else
		{
			self.metadata.createAudioNode(configuration, languageData, &mp4UrlBorrow.as_ref().unwrap(), durationInSeconds, self.volume)
		}
	}
	
	#[inline(always)]
	fn width_default() -> u16
	{
		400
	}
	
	#[inline(always)]
	fn height_default() -> u16
	{
		60
	}
}
