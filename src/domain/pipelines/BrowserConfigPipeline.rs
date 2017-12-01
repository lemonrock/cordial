// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// See https://www.w3.org/TR/appmanifest/#webappmanifest-dictionary
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct BrowserConfigPipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_downloadable_false_default")] is_downloadable: bool,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default = "WebAppManifestPipeline::language_aware_default")] language_aware: bool,
	#[serde(default)] input_format: Option<BrowserConfigInputFormat>,
	
	#[serde(default)] pub(crate) tile_url: ResourceUrl,
	#[serde(default)] pub(crate) tile_color: [u8; 3],
	#[serde(default)] pub(crate) badge_url: Option<ResourceUrl>,
	#[serde(default)] pub(crate) badge_poll_frequency: BrowserConfigPollFrequencyInMinutes,
	#[serde(default)] pub(crate) notification_urls: [Option<ResourceUrl>; 5],
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
			tile_color: [0; 3],
			badge_url: None,
			badge_poll_frequency: BrowserConfigPollFrequencyInMinutes::default(),
			notification_urls: [None, None, None, None, None],
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
		DependsOnOthersEgStylesheet
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
	fn execute(&self, resources: &Resources, _inputContentFilePath: &Path, resourceUrl: &ResourceUrl, _handlebars: &HandlebarsWrapper, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, configuration: &Configuration, _rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, _siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<PipelineResource>, CordialError>
	{
		let url = resourceUrl.replaceFileNameExtension(".xml").url(languageData)?;
		
		const CanBeCompressed: bool = true;
		let headers = headerGenerator.generateHeadersForAsset(CanBeCompressed, self.max_age_in_seconds, self.is_downloadable, &url)?;
		
		let body = self.body(resources, configuration.fallbackIso639Dash1Alpha2Language(), Some(languageData.iso639Dash1Alpha2Language))?;
		
		Ok(vec![(url, hashmap! { default => Rc::new(UrlDataDetails::generic(&body)) }, StatusCode::Ok, ContentType(mime::TEXT_XML), headers, body, None, CanBeCompressed)])
	}
}

impl BrowserConfigPipeline
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn body(&self, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<Vec<u8>, CordialError>
	{
		let emptyAttributes = [];
		
		let mut eventWriter = Self::createEventWriter();
		
		eventWriter.writeBasicXmlDocumentPreamble()?;
		
		let namespace = Namespace
		(
			btreemap!
			{
			}
		);
		
		eventWriter.writeWithinElement(Name::local("browserconfig"), &namespace, &emptyAttributes, |eventWriter|
		{
			eventWriter.writeWithinElement(Name::local("msapplication"), &namespace, &emptyAttributes, |eventWriter|
			{
				eventWriter.writeWithinElement(Name::local("tile"), &namespace, &emptyAttributes, |mut eventWriter|
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
					eventWriter.writeWithinElement(Name::local("badge"), &namespace, &emptyAttributes, |mut eventWriter|
					{
						Self::writePollElement(&mut eventWriter, &namespace, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, "polling-uri", badge_url)?;
						Self::writeFrequencyElement(&mut eventWriter, &namespace, self.badge_poll_frequency)
					})?;
				}
				
				if self.notification_urls[0].is_some()
				{
					eventWriter.writeWithinElement(Name::local("notification"), &namespace, &emptyAttributes, |mut eventWriter|
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
		let mut keepWriting = true;
		for index in 0..4
		{
			if keepWriting
			{
				keepWriting = self.writePollNotificationElement(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, eventWriter, namespace, index)?;
			}
			else
			{
				if self.notification_urls[index].is_some()
				{
					return Err(CordialError::Configuration("Non-contiguous poll URIs".to_owned()));
				}
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn writePollNotificationElement(&self, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace, index: usize) -> Result<bool, CordialError>
	{
		match self.notification_urls[index]
		{
			None => Ok(false),
			Some(ref resourceUrl) =>
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
				
				Self::writePollElement(eventWriter, namespace, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, elementName, resourceUrl)?;
				
				Ok(true)
			}
		}
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
		eventWriter.writeTextElement(&namespace, &[], Name::local("TileColor"), &format!("#{:02X}{:02X}{:02X}", self.tile_color[0], self.tile_color[1], self.tile_color[2]))
	}
	
	#[inline(always)]
	fn writeFrequencyElement(eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace, frequency: BrowserConfigPollFrequencyInMinutes) -> Result<(), CordialError>
	{
		eventWriter.writeTextElement(&namespace, &[], Name::local("frequency"), frequency.to_str())
	}
	
	#[inline(always)]
	fn writePollCycleElement(&self, eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace) -> Result<(), CordialError>
	{
		eventWriter.writeTextElement(&namespace, &[], Name::local("cycle"), self.notification_poll_cycle.to_str())
	}
	
	#[inline(always)]
	fn writeEmptyElementWithSrcAttribute(eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace, name: &str, urlData: Rc<UrlData>) -> Result<(), CordialError>
	{
		let attributes =
		[
			XmlAttribute::new(Name::local("src"), urlData.url_str()),
		];
		eventWriter.writeEmptyElement(&namespace, &attributes, Name::local(name))
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
	fn createEventWriter<'a>() -> EventWriter<Vec<u8>>
	{
		let configuration = EmitterConfig
		{
			line_separator: Cow::Borrowed(""),
			indent_string: Cow::Borrowed(""),
			perform_indent: false,
			perform_escaping: true,
			write_document_declaration: true,
			normalize_empty_elements: true,
			cdata_to_characters: true,
			keep_element_names_stack: true,
			autopad_comments: false,
		};
		configuration.create_writer(Vec::with_capacity(4096))
	}
	
	#[inline(always)]
	fn language_aware_default() -> bool
	{
		true
	}
}
