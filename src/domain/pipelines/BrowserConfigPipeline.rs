// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// See https://www.w3.org/TR/appmanifest/#webappmanifest-dictionary
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct BrowserConfigPipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_downloadable_false_default")] is_downloadable: bool,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default = "WebAppManifestPipeline::language_aware_default")] language_aware: bool,
	#[serde(default)] input_format: Option<BrowserConfigInputFormat>,
	
	#[serde(default)] pub(crate) tile_url: ResourceUrl,
	#[serde(default)] pub(crate) tile_color: HexadecimalColor,
	#[serde(default)] pub(crate) badge_url: Option<ResourceUrl>,
	#[serde(default)] pub(crate) badge_poll_frequency: BrowserConfigPollFrequencyInMinutes,
	#[serde(default)] pub(crate) notification_urls: ArrayVec<[ResourceUrl; 5]>,
	#[serde(default)] pub(crate) notification_poll_frequency: BrowserConfigPollFrequencyInMinutes,
	#[serde(default)] pub(crate) notification_poll_cycle: BrowserConfigPollNotificationCycle,
}

impl Default for BrowserConfigPipeline
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			max_age_in_seconds: max_age_in_seconds_long_default(),
			is_downloadable: is_downloadable_false_default(),
			is_versioned: Self::language_aware_default(),
			language_aware: false,
			input_format: None,
			
			tile_url: Default::default(),
			tile_color: Default::default(),
			badge_url: None,
			badge_poll_frequency: BrowserConfigPollFrequencyInMinutes::default(),
			notification_urls: Default::default(),
			notification_poll_frequency: BrowserConfigPollFrequencyInMinutes::default(),
			notification_poll_cycle: BrowserConfigPollNotificationCycle::default(),
		}
	}
}

impl Pipeline for BrowserConfigPipeline
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
		(self.is_versioned, self.language_aware)
	}
	
	#[inline(always)]
	fn execute(&self, resources: &Resources, _inputContentFilePath: &Path, resourceUrl: &ResourceUrl, _handlebars: &HandlebarsWrapper, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, configuration: &Configuration, _rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, _siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<PipelineResponse>, CordialError>
	{
		let url = resourceUrl.replaceFileNameExtension(".xml").url(languageData)?;
		
		const CanBeCompressed: bool = true;
		let headers = headerGenerator.generateHeadersForAsset(CanBeCompressed, self.max_age_in_seconds, self.is_downloadable, &url)?;
		
		let body = self.body(resources, configuration.fallbackIso639Dash1Alpha2Language(), Some(languageData.iso639Dash1Alpha2Language))?;
		
		Ok(vec![(url, hashmap! { default => Rc::new(UrlDataDetails::generic(&body)) }, StatusCode::Ok, content_type_application_xml_utf8(), headers, ResponseBody::utf8(body), None, CanBeCompressed)])
	}
}

impl BrowserConfigPipeline
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn body(&self, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<Vec<u8>, CordialError>
	{
		let emptyAttributes = [];
		
		let mut eventWriter = minifyingVecEventWriter();
		
		eventWriter.writeBasicXmlDocumentPreamble()?;
		
		let namespace = Namespace::empty();
		
		eventWriter.writeWithinLocalElement("browserconfig", &namespace, &emptyAttributes, |eventWriter|
		{
			eventWriter.writeWithinLocalElement("msapplication", &namespace, &emptyAttributes, |eventWriter|
			{
				eventWriter.writeWithinLocalElement("tile", &namespace, &emptyAttributes, |mut eventWriter|
				{
					// TODO: Almost. In fact Microsoft recommends to use larger pictures. This is to present high resolution pictures to the user even when the desktop is scaled up. Therefore the recommended sizes are 128x128, 270x270, 558x558 and 558x270.
					self.writeTileIconElement(&mut eventWriter, &namespace, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, 70, 70, 128, 128)?;
					self.writeTileIconElement(&mut eventWriter, &namespace, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, 150, 150, 270, 270)?;
					self.writeTileIconElement(&mut eventWriter, &namespace, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, 310, 150, 558, 270)?;
					self.writeTileIconElement(&mut eventWriter, &namespace, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, 310, 310, 558, 558)?;
					self.writeTileColorElement(&mut eventWriter, &namespace)
				})?;
				
				if let Some(ref badge_url) = self.badge_url
				{
					eventWriter.writeWithinLocalElement("badge", &namespace, &emptyAttributes, |mut eventWriter|
					{
						Self::writePollElement(&mut eventWriter, &namespace, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, "polling-uri", badge_url)?;
						Self::writeFrequencyElement(&mut eventWriter, &namespace, self.badge_poll_frequency)
					})?;
				}
				
				if !self.notification_urls.is_empty()
				{
					eventWriter.writeWithinLocalElement("notification", &namespace, &emptyAttributes, |mut eventWriter|
					{
						self.writePollNotificationElements(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, &mut eventWriter, &namespace)?;
						Self::writeFrequencyElement(&mut eventWriter, &namespace, self.notification_poll_frequency)?;
						self.writePollCycleElement(&mut eventWriter, &namespace)
					})?;
				}
				
				Ok(())
			})
		})?;
		
		let mut body = eventWriter.into_inner();
		body.shrink_to_fit();
		
		Ok(body)
	}
	
	#[inline(always)]
	fn writePollNotificationElements(&self, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace)  -> Result<(), CordialError>
	{
		let mut index = 0;
		for notificationUrl in self.notification_urls.iter()
		{
			let elementName = match index
			{
				0 => "polling-uri",
				1 => "polling-uri1",
				2 => "polling-uri2",
				3 => "polling-uri3",
				4 => "polling-uri4",
				_ => unreachable!(),
			};
			
			Self::writePollElement(eventWriter, namespace, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, elementName, notificationUrl)?;
			
			index += 1;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn writePollElement(eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, elementName: &str, resourceUrl: &ResourceUrl) -> Result<(), CordialError>
	{
		let urlData = Self::xmlUrlData(resourceUrl, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		
		Self::writeEmptyElementWithSrcAttribute(eventWriter, namespace, &elementName, urlData)
	}
	
	#[inline(always)]
	fn writeTileIconElement(&self, eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, width: u32, height: u32, iconWidth: u32, iconHeight: u32) -> Result<(), CordialError>
	{
		let elementName = if width == height
		{
			format!("square{}x{}logo", width, height)
		}
		else
		{
			format!("wide{}x{}logo", width, height)
		};
		
		let urlData = Self::pngUrlData(&self.tile_url, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, iconWidth, iconHeight)?;
		
		Self::writeEmptyElementWithSrcAttribute(eventWriter, namespace, &elementName, urlData)
	}
	
	#[inline(always)]
	fn writeTileColorElement(&self, eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace) -> Result<(), CordialError>
	{
		eventWriter.writeUnprefixedTextElementString(&namespace, &[], "TileColor", self.tile_color.toStringWithHashPrefix())
	}
	
	#[inline(always)]
	fn writeFrequencyElement(eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace, frequency: BrowserConfigPollFrequencyInMinutes) -> Result<(), CordialError>
	{
		eventWriter.writeTextElement(&namespace, &[], "frequency".xml_local_name(), frequency.to_str())
	}
	
	#[inline(always)]
	fn writePollCycleElement(&self, eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace) -> Result<(), CordialError>
	{
		eventWriter.writeTextElement(&namespace, &[], "cycle".xml_local_name(), self.notification_poll_cycle.to_str())
	}
	
	#[inline(always)]
	fn writeEmptyElementWithSrcAttribute(eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace, name: &str, urlData: Rc<UrlData>) -> Result<(), CordialError>
	{
		eventWriter.writeEmptyElement(&namespace, &[ "src".xml_url_from_UrlData_attribute(&urlData) ], name.xml_local_name())
	}
	
	#[inline(always)]
	fn pngUrlData(tileUrl: &ResourceUrl, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, width: u32, height: u32) -> Result<Rc<UrlData>, CordialError>
	{
		let urlData = ResourceReference
		{
			resource: tileUrl.clone(),
			tag: ResourceTag::width_height_image(width, height)
		}.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		urlData.validateIsPng()?;
		Ok(urlData)
	}
	
	#[inline(always)]
	fn xmlUrlData(xmlUrl: &ResourceUrl, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<Rc<UrlData>, CordialError>
	{
		let urlData = ResourceReference
		{
			resource: xmlUrl.clone(),
			tag: ResourceTag::default
		}.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		urlData.validateIsXml()?;
		Ok(urlData)
	}
	
	#[inline(always)]
	fn language_aware_default() -> bool
	{
		true
	}
}
