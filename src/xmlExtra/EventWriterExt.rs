// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait EventWriterExt
{
	#[inline(always)]
	fn writeBasicXmlDocumentPreamble(&mut self) -> Result<(), CordialError>;
	
	#[inline(always)]
	fn writeProcessingInstruction(&mut self, name: &str, data: Option<&str>) -> Result<(), CordialError>;
	
	#[inline(always)]
	fn writeSimpleStartElement<'a>(&mut self, name: Name<'a>, namespace: &Namespace, attributes: &[Attribute<'a>]) -> Result<(), CordialError>;
	
	#[inline(always)]
	fn writeText(&mut self, text: &str) -> Result<(), CordialError>;
	
	#[inline(always)]
	fn writeCData(&mut self, text: &str) -> Result<(), CordialError>;
	
	#[inline(always)]
	fn writeEndElement<'a>(&mut self, name: Name<'a>) -> Result<(), CordialError>;
	
	#[inline(always)]
	fn writeEmptyElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>) -> Result<(), CordialError>;
	
	#[inline(always)]
	fn writeTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>, text: &str) -> Result<(), CordialError>;
	
	#[inline(always)]
	fn writeCDataElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>, text: &str) -> Result<(), CordialError>;
	
	#[inline(always)]
	fn writeUnprefixedTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, text: &str) -> Result<(), CordialError>;
	
	#[inline(always)]
	fn writeUnprefixedTextElementString<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, text: String) -> Result<(), CordialError>
	{
		self.writeUnprefixedTextElement(namespace, attributes, name, &text)
	}
	
	#[inline(always)]
	fn writePrefixedTextElementString<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], prefix: &str, name: &str, text: String) -> Result<(), CordialError>
	{
		self.writePrefixedTextElement(namespace, attributes, prefix, name, &text)
	}
	
	#[inline(always)]
	fn writeUnprefixedTextElementUrl<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, url: &Url) -> Result<(), CordialError>
	{
		self.writeUnprefixedTextElement(namespace, attributes, name, url.as_ref())
	}
	
	#[inline(always)]
	fn writeUnprefixedTextElementEMailAddress<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, emailAddress: &EMailAddress) -> Result<(), CordialError>
	{
		self.writeUnprefixedTextElementString(namespace, attributes, name, emailAddress.to_string())
	}
	
	#[inline(always)]
	fn writeUnprefixedTextElementLanguageCode<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		self.writeUnprefixedTextElement(namespace, attributes, name, iso639Dash1Alpha2Language.to_iso_639_1_alpha_2_language_code())
	}
	
	#[inline(always)]
	fn writeUnprefixedTextElementRfc2822<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, dateTime: DateTime<Utc>) -> Result<(), CordialError>
	{
		self.writeUnprefixedTextElementString(namespace, attributes, name, dateTime.to_rfc2822())
	}
	
	#[inline(always)]
	fn writeUnprefixedTextElementRfc3339<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, dateTime: DateTime<Utc>) -> Result<(), CordialError>
	{
		self.writeUnprefixedTextElementString(namespace, attributes, name, dateTime.to_rfc3339())
	}
	
	#[inline(always)]
	fn writePrefixedTextElementRfc3339<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], prefix: &str, name: &str, dateTime: DateTime<Utc>) -> Result<(), CordialError>
	{
		self.writePrefixedTextElementString(namespace, attributes, prefix, name, dateTime.to_rfc3339())
	}
	
	#[inline(always)]
	fn writeUnprefixedTextElementU32<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, value: u32) -> Result<(), CordialError>
	{
		self.writeUnprefixedTextElementString(namespace, attributes, name, format!("{}", value))
	}
	
	#[inline(always)]
	fn writeUnprefixedTextElementU64<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, value: u64) -> Result<(), CordialError>
	{
		self.writeUnprefixedTextElementString(namespace, attributes, name, format!("{}", value))
	}
	
	#[inline(always)]
	fn writePrefixedTextElementU64<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], prefix: &str, name: &str, value: u64) -> Result<(), CordialError>
	{
		self.writePrefixedTextElementString(namespace, attributes, prefix, name, format!("{}", value))
	}
	
	#[inline(always)]
	fn writePrefixedTextElementU16<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], prefix: &str, name: &str, value: u16) -> Result<(), CordialError>
	{
		self.writePrefixedTextElementString(namespace, attributes, prefix, name, format!("{}", value))
	}
	
	#[inline(always)]
	fn writePrefixedTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], prefix: &str, name: &str, text: &str) -> Result<(), CordialError>;
	
	#[inline(always)]
	fn writeWithinElement<'a, F: FnOnce(&mut Self) -> Result<(), CordialError>>(&mut self, name: Name<'a>, namespace: &Namespace, attributes: &[Attribute<'a>], children: F) -> Result<(), CordialError>;
	
	#[inline(always)]
	fn writeWithinLocalElement<'a, F: FnOnce(&mut Self) -> Result<(), CordialError>>(&mut self, name: &str, namespace: &Namespace, attributes: &[Attribute<'a>], children: F) -> Result<(), CordialError>
	{
		self.writeWithinElement(name.xml_local_name(), namespace, attributes, children)
	}
}

impl<W: Write> EventWriterExt for EventWriter<W>
{
	#[inline(always)]
	fn writeBasicXmlDocumentPreamble(&mut self) -> Result<(), CordialError>
	{
		self.write(XmlEvent::StartDocument
		{
			version: XmlVersion::Version10,
			encoding: Some("UTF-8"),
			standalone: None,
		})?;
		Ok(())
	}
	
	#[inline(always)]
	fn writeProcessingInstruction(&mut self, name: &str, data: Option<&str>) -> Result<(), CordialError>
	{
		self.write(XmlEvent::processing_instruction(name, data))?;
		Ok(())
	}
	
	#[inline(always)]
	fn writeSimpleStartElement<'a>(&mut self, name: Name<'a>, namespace: &Namespace, attributes: &[Attribute<'a>]) -> Result<(), CordialError>
	{
		self.write(XmlEvent::StartElement
		{
			name,
			attributes: Cow::Borrowed(attributes),
			namespace: Cow::Borrowed(namespace),
		})?;
		Ok(())
	}
	
	#[inline(always)]
	fn writeText(&mut self, text: &str) -> Result<(), CordialError>
	{
		self.write(XmlEvent::Characters(text))?;
		Ok(())
	}
	
	#[inline(always)]
	fn writeCData(&mut self, text: &str) -> Result<(), CordialError>
	{
		self.write(XmlEvent::CData(text))?;
		Ok(())
	}
	
	#[inline(always)]
	fn writeEndElement<'a>(&mut self, name: Name<'a>) -> Result<(), CordialError>
	{
		self.write(XmlEvent::EndElement
		{
			name: Some(name),
		})?;
		Ok(())
	}
	
	#[inline(always)]
	fn writeEmptyElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>) -> Result<(), CordialError>
	{
		self.writeSimpleStartElement(name, namespace, attributes)?;
		self.writeEndElement(name)?;
		Ok(())
	}
	
	#[inline(always)]
	fn writeTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>, text: &str) -> Result<(), CordialError>
	{
		self.writeSimpleStartElement(name, namespace, attributes)?;
		{
			self.writeText(text)?;
		}
		self.writeEndElement(name)?;
		Ok(())
	}
	
	#[inline(always)]
	fn writeCDataElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>, text: &str) -> Result<(), CordialError>
	{
		self.writeSimpleStartElement(name, namespace, attributes)?;
		{
			self.writeCData(text)?;
		}
		self.writeEndElement(name)?;
		Ok(())
	}
	
	#[inline(always)]
	fn writeUnprefixedTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, text: &str) -> Result<(), CordialError>
	{
		self.writeTextElement(namespace, attributes, name.xml_local_name(), text)?;
		Ok(())
	}
	
	#[inline(always)]
	fn writePrefixedTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], prefix: &str, name: &str, text: &str) -> Result<(), CordialError>
	{
		self.writeTextElement(namespace, attributes, prefix.prefixes_xml_name(name), text)?;
		Ok(())
	}
	
	#[inline(always)]
	fn writeWithinElement<'a, F: FnOnce(&mut Self) -> Result<(), CordialError>>(&mut self, name: Name<'a>, namespace: &Namespace, attributes: &[Attribute<'a>], children: F) -> Result<(), CordialError>
	{
		self.writeSimpleStartElement(name, namespace, attributes)?;
		{
			children(self)?;
		}
		self.writeEndElement(name)?;
		Ok(())
	}
}
