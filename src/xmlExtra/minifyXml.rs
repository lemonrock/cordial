// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn minifyXml(source: String) -> Result<Vec<u8>, CordialError>
{
	let bytes = source.into_bytes();
	
	let eventReader = ParserConfig
	{
		trim_whitespace: true,
		whitespace_to_characters: true,
		cdata_to_characters: true,
		ignore_comments: true,
		coalesce_characters: true,
		extra_entities: Default::default(),
		ignore_end_of_stream: false,
	}.create_reader(bytes.as_slice());
	
	let mut eventWriter = minifyingVecEventWriter();
	
	for readerXmlEvent in eventReader
	{
		let readerXmlEvent = readerXmlEvent?;
		
		use self::ReaderXmlEvent::*;
		
		match readerXmlEvent
		{
			StartDocument { version, encoding, standalone } => eventWriter.write(XmlEvent::StartDocument
			{
				version,
				encoding: Some(encoding.as_str()),
				standalone,
			})?,
			
			ProcessingInstruction { name, data } => if let Some(ref data) = data
			{
				eventWriter.write(XmlEvent::processing_instruction(&name, Some(data.as_str())))?
			}
			else
			{
				eventWriter.write(XmlEvent::processing_instruction(&name, None))?
			},
			
			StartElement { name, attributes, namespace } =>
			{
				let attributes: Vec<Attribute> = attributes.iter().map(|attribute| attribute.borrow()).collect();
				
				eventWriter.write(XmlEvent::StartElement
				{
					name: name.borrow(),
					attributes: Cow::from(attributes),
					namespace: Cow::Owned(namespace),
				})?
			}
			
			EndElement { name } => eventWriter.write(XmlEvent::EndElement
			{
				name: Some(name.borrow()),
			})?,
			
			CData(string) => eventWriter.write(XmlEvent::cdata(&string))?,
			
			Comment(_) => (),
			
			Characters(characters) => eventWriter.write(XmlEvent::cdata(&characters))?,
			
			Whitespace(_) => (),
			
			EndDocument => (),
		}
	}
	
	let mut bytes = eventWriter.into_inner();
	bytes.shrink_to_fit();
	Ok(bytes)
}
