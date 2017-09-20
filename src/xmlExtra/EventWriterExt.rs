// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait EventWriterExt
{
	#[inline(always)]
	fn writeBasicXmlDocumentPreamble(&mut self) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeProcessingInstruction(&mut self, name: &str, data: Option<&str>) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeSimpleStartElement<'a>(&mut self, name: Name<'a>, namespace: &Namespace, attributes: &[Attribute<'a>]) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeText(&mut self, text: &str) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeCData(&mut self, text: &str) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeEndElement<'a>(&mut self, name: Name<'a>) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeEmptyElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>, text: &str) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeCDataElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>, text: &str) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeUnprefixedTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, text: &str) -> XmlWriterResult;
	
	#[inline(always)]
	fn writePrefixedTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], prefix: &str, name: &str, text: &str) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeWithinElement<'a, F: FnOnce(&mut Self) -> XmlWriterResult>(&mut self, name: Name<'a>, namespace: &Namespace, attributes: &[Attribute<'a>], children: F) -> XmlWriterResult;
}

impl<W: Write> EventWriterExt for EventWriter<W>
{
	#[inline(always)]
	fn writeBasicXmlDocumentPreamble(&mut self) -> XmlWriterResult
	{
		self.write(XmlEvent::StartDocument
		{
			version: XmlVersion::Version10,
			encoding: Some("UTF-8"),
			standalone: None,
		})
	}
	
	#[inline(always)]
	fn writeProcessingInstruction(&mut self, name: &str, data: Option<&str>) -> XmlWriterResult
	{
		self.write(XmlEvent::processing_instruction(name, data))
	}
	
	#[inline(always)]
	fn writeSimpleStartElement<'a>(&mut self, name: Name<'a>, namespace: &Namespace, attributes: &[Attribute<'a>]) -> XmlWriterResult
	{
		self.write(XmlEvent::StartElement
		{
			name,
			attributes: Cow::Borrowed(attributes),
			namespace: Cow::Borrowed(namespace),
		})
	}
	
	#[inline(always)]
	fn writeText(&mut self, text: &str) -> XmlWriterResult
	{
		self.write(XmlEvent::Characters(text))
	}
	
	#[inline(always)]
	fn writeCData(&mut self, text: &str) -> XmlWriterResult
	{
		self.write(XmlEvent::CData(text))
	}
	
	#[inline(always)]
	fn writeEndElement<'a>(&mut self, name: Name<'a>) -> XmlWriterResult
	{
		self.write(XmlEvent::EndElement
		{
			name: Some(name),
		})
	}
	
	#[inline(always)]
	fn writeEmptyElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>) -> XmlWriterResult
	{
		self.writeSimpleStartElement(name, namespace, attributes)?;
		self.writeEndElement(name)
	}
	
	#[inline(always)]
	fn writeTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>, text: &str) -> XmlWriterResult
	{
		self.writeSimpleStartElement(name, namespace, attributes)?;
		{
			self.writeText(text)?;
		}
		self.writeEndElement(name)
	}
	
	#[inline(always)]
	fn writeCDataElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>, text: &str) -> XmlWriterResult
	{
		self.writeSimpleStartElement(name, namespace, attributes)?;
		{
			self.writeCData(text)?;
		}
		self.writeEndElement(name)
	}
	
	#[inline(always)]
	fn writeUnprefixedTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, text: &str) -> XmlWriterResult
	{
		self.writeTextElement(namespace, attributes, Name::local(name), text)
	}
	
	#[inline(always)]
	fn writePrefixedTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], prefix: &str, name: &str, text: &str) -> XmlWriterResult
	{
		self.writeTextElement(namespace, attributes, Name::prefixed(name, prefix), text)
	}
	
	#[inline(always)]
	fn writeWithinElement<'a, F: FnOnce(&mut Self) -> XmlWriterResult>(&mut self, name: Name<'a>, namespace: &Namespace, attributes: &[Attribute<'a>], children: F) -> XmlWriterResult
	{
		self.writeSimpleStartElement(name, namespace, attributes)?;
		{
			children(self)?;
		}
		self.writeEndElement(name)
	}
}
