// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


trait XmlWriterExt
{
	#[inline(always)]
	fn writeBasicXmlDocumentPreamble(&mut self) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeSimpleStartElement<'a>(&mut self, name: Name<'a>, namespace: &Namespace, attributes: &[Attribute<'a>]) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeText(&mut self, text: &str) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeEndElement<'a>(&mut self, name: Name<'a>) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeEmptyElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>, text: &str) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeUnprefixedTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, text: &str) -> XmlWriterResult;
	
	#[inline(always)]
	fn writePrefixedTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], prefix: &str, name: &str, text: &str) -> XmlWriterResult;
	
	#[inline(always)]
	fn writeWithinElement<'a, F: FnMut() -> XmlWriterResult>(&mut self, name: Name<'a>, namespace: &Namespace, attributes: &[Attribute<'a>], children: F) -> XmlWriterResult;
}

impl<W: Write> XmlWriterExt for XmlWriter<W>
{
	#[inline(always)]
	fn writeBasicXmlDocumentPreamble(&mut self) -> XmlWriterResult
	{
		self.write(XmlEvent::StartDocument
		{
			version: XmlVersion::Version10,
			encoding: Some("UTF-8"),
			standalone: None,
		})?
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
		eventWriter.writeSimpleStartElement(name, namespace, attributes)?;
		eventWriter.writeEndElement(name)
	}
	
	#[inline(always)]
	fn writeTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: Name<'a>, text: &str) -> XmlWriterResult
	{
		eventWriter.writeSimpleStartElement(name, namespace, attributes)?;
		{
			if !text.is_empty()
			{
				eventWriter.writeText(text)?;
			}
		}
		eventWriter.writeEndElement(name)
	}
	
	#[inline(always)]
	fn writeUnprefixedTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], name: &str, text: &str) -> XmlWriterResult
	{
		self.writeTextElement(Name::local(name), namespace, emptyAttributes, text)
	}
	
	#[inline(always)]
	fn writePrefixedTextElement<'a>(&mut self, namespace: &Namespace, attributes: &[Attribute<'a>], prefix: &str, name: &str, text: &str) -> XmlWriterResult
	{
		self.writeTextElement(Name::prefixed(name, prefix), namespace, emptyAttributes, text)
	}
	
	#[inline(always)]
	fn writeWithinElement<'a, F: FnMut() -> XmlWriterResult>(&mut self, name: Name<'a>, namespace: &Namespace, attributes: &[Attribute<'a>], children: F) -> XmlWriterResult
	{
		eventWriter.writeSimpleStartElement(name, namespace, emptyAttributes)?;
		{
			children()?;
		}
		eventWriter.writeEndElement(name)
	}
}
