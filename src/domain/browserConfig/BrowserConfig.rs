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
				eventWriter.writeWithinElement(Name::local("tile"), &namespace, &emptyAttributes, |eventWriter|
				{
					let borrowed = Self::pngUrlData(&self.tile_url, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, 70, 70)?;
					let attributes =
					[
						XmlAttribute::new(Name::local("src"), borrowed.url_str()),
					];
					eventWriter.writeEmptyElement(&namespace, &attributes, Name::local("square70x70logo"))?;
					
					let borrowed = Self::pngUrlData(&self.tile_url, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, 150, 150)?;
					let attributes =
					[
						XmlAttribute::new(Name::local("src"), borrowed.url_str()),
					];
					eventWriter.writeEmptyElement(&namespace, &attributes, Name::local("square150x150logo"))?;
					
					let borrowed = Self::pngUrlData(&self.tile_url, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, 310, 150)?;
					let attributes =
					[
						XmlAttribute::new(Name::local("src"), borrowed.url_str()),
					];
					eventWriter.writeEmptyElement(&namespace, &attributes, Name::local("wide310x150logo"))?;
					
					let borrowed = Self::pngUrlData(&self.tile_url, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, 310, 310)?;
					let attributes =
					[
						XmlAttribute::new(Name::local("src"), borrowed.url_str()),
					];
					eventWriter.writeEmptyElement(&namespace, &attributes, Name::local("square310x310logo"))?;
					
					eventWriter.writeTextElement(&namespace, &emptyAttributes, Name::local("TileColor"), &format!("#{:02X}{:02X}{:02X}", self.tile_color[0], self.tile_color[1], self.tile_color[2]))
				})?;
				
				if let Some(ref badge_url) = self.badge_url
				{
					eventWriter.writeWithinElement(Name::local("badge"), &namespace, &emptyAttributes, |eventWriter|
					{
						let borrowed = Self::xmlUrlData(badge_url, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
						let attributes =
						[
							XmlAttribute::new(Name::local("src"), borrowed.url_str()),
						];
						eventWriter.writeEmptyElement(&namespace, &attributes, Name::local("polling-uri"))?;
						
						eventWriter.writeTextElement(&namespace, &emptyAttributes, Name::local("frequency"), self.badge_poll_frequency.to_str())
					})?;
				}
				
				if self.notification_urls[0].is_some()
				{
					eventWriter.writeWithinElement(Name::local("notification"), &namespace, &emptyAttributes, |mut eventWriter|
					{
						let mut keepWriting = true;
						for index in 0..4
						{
							if keepWriting
							{
								keepWriting = self.notificationUrlData(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, &mut eventWriter, &namespace, index)?;
							}
							else
							{
								if self.notification_urls[index].is_some()
								{
									return Err(CordialError::Configuration("Non-contiguous poll URIs".to_owned()));
								}
							}
						}
						
						eventWriter.writeTextElement(&namespace, &emptyAttributes, Name::local("frequency"), self.notification_poll_frequency.to_str())?;
						
						eventWriter.writeTextElement(&namespace, &emptyAttributes, Name::local("cycle"), self.notification_poll_cycle.to_str())
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
	fn notificationUrlData(&self, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, eventWriter: &mut EventWriter<Vec<u8>>, namespace: &Namespace, index: usize) -> Result<bool, CordialError>
	{
		match self.notification_urls[index]
		{
			None => Ok(false),
			Some(ref resourceUrl) =>
			{
				let borrowed = Self::xmlUrlData(resourceUrl, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
				let attributes =
				[
					XmlAttribute::new(Name::local("src"), borrowed.url_str()),
				];
				
				let elementName = match index
				{
					0 => "polling-uri",
					1 => "polling-uri1",
					2 => "polling-uri2",
					3 => "polling-uri3",
					4 => "polling-uri4",
					_ => unreachable!(),
				};
				
				eventWriter.writeEmptyElement(namespace, &attributes, Name::local(elementName))?;
				
				Ok(true)
			}
		}
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
