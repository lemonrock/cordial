// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct RssImage
{
	pub(crate) width: u32,
	pub(crate) height: u32,
	pub(crate) url: Rc<Url>,
	pub(crate) fileSize: u64,
	pub(crate) mimeType: Mime,
	pub(crate) imageAbstract: Rc<ImageAbstract>,
	pub(crate) credit: FullName,
	pub(crate) iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language,
}

impl RssImage
{
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		let fileSize = format!("{}", self.fileSize);
		let width = format!("{}", self.width);
		let height = format!("{}", self.height);
		let url = self.url.as_ref().as_str();
		let mimeType = self.mimeType.as_ref();
		
		let enclosureAttributes =
		[
			XmlAttribute::new(Name::local("url"), url),
			XmlAttribute::new(Name::local("length"), &fileSize),
			XmlAttribute::new(Name::local("type"), mimeType),
		];
		eventWriter.writeEmptyElement(namespace, &enclosureAttributes, Name::local("enclosure"))?;
		
		let contentAttributes =
		[
			XmlAttribute::new(Name::local("url"), url),
			XmlAttribute::new(Name::local("medium"), "image"),
			XmlAttribute::new(Name::local("height"), &height),
			XmlAttribute::new(Name::local("width"), &width),
			XmlAttribute::new(Name::local("fileSize"), &fileSize),
			XmlAttribute::new(Name::local("type"), mimeType),
			XmlAttribute::new(Name::local("lang"), self.iso639Dash1Alpha2Language.to_iso_639_1_alpha_2_language_code()),
		];
		eventWriter.writeWithinElement(Name::prefixed("content", "media"), &namespace, &contentAttributes, |eventWriter|
		{
			eventWriter.writeTextElement(namespace, &emptyAttributes, Name::prefixed("description", "media"), &self.imageAbstract.alt)?;
			
			eventWriter.writeTextElement(namespace, &emptyAttributes, Name::prefixed("credit", "media"), &self.credit)?;
			
			let thumbnailAttributes =
			[
				XmlAttribute::new(Name::local("width"), &width),
				XmlAttribute::new(Name::local("height"), &height),
				XmlAttribute::new(Name::local("url"), url),
			];
			eventWriter.writeEmptyElement(namespace, &thumbnailAttributes, Name::prefixed("thumbnail", "media"))
		})
	}
}
