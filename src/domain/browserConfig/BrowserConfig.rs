// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// See https://msdn.microsoft.com/library/dn320426(v=vs.85).aspx
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct BrowserConfig
{
	#[serde(default)] pub(crate) tile_url: ResourceUrl,
	#[serde(default)] pub(crate) tile_color: [u8; 3],
	#[serde(default)] pub(crate) badge_url: Option<ResourceUrl>,
	#[serde(default)] pub(crate) badge_poll_frequency: BrowserConfigPollFrequencyInMinutes,
	#[serde(default)] pub(crate) notification_urls: [Option<ResourceUrl>; 5],
	#[serde(default)] pub(crate) notification_poll_frequency: BrowserConfigPollFrequencyInMinutes,
	#[serde(default)] pub(crate) notification_poll_cycle: BrowserConfigPollNotificationCycle,
}

impl BrowserConfig
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn to_xml(&self, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<Vec<u8>, CordialError>
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
					self.writeTileIconElement(&mut eventWriter, &namespace, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, 70, 70)?;
					self.writeTileIconElement(&mut eventWriter, &namespace, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, 150, 150)?;
					self.writeTileIconElement(&mut eventWriter, &namespace, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, 310, 150)?;
					self.writeTileIconElement(&mut eventWriter, &namespace, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, 310, 310)?;
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
	fn writeFrequencyElement(eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace, frequency: BrowserConfigPollFrequencyInMinutes) -> Result<(), CordialError>
	{
		eventWriter.writeTextElement(&namespace, &[], Name::local("frequency"), frequency.to_str())
	}
	
	#[inline(always)]
	fn writeTileIconElement(&self, eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, width: u32, height: u32) -> Result<(), CordialError>
	{
		let elementName = if width == height
		{
			format!("square{}x{}logo", width, height)
		}
		else
		{
			format!("wide{}x{}logo", width, height)
		};
		
		let urlData = Self::pngUrlData(&self.tile_url, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, width, height)?;
		
		Self::writeEmptyElementWithSrcAttribute(eventWriter, namespace, &elementName, urlData)
	}
	
	#[inline(always)]
	fn writeTileColorElement(&self, eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace) -> Result<(), CordialError>
	{
		eventWriter.writeTextElement(&namespace, &[], Name::local("TileColor"), &format!("#{:02X}{:02X}{:02X}", self.tile_color[0], self.tile_color[1], self.tile_color[2]))
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
}
