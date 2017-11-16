// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ResourceUrl<'a>(pub(crate) Rc<Cow<'a, str>>);

impl Default for ResourceUrl<'static>
{
	#[inline(always)]
	fn default() -> Self
	{
		ResourceUrl::str("/")
	}
}

impl<'a> Display for ResourceUrl<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.0.fmt(f)
	}
}

impl<'de, 'a> Deserialize<'de> for ResourceUrl<'a>
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct ResourceUrlVisitor<'a>(PhantomData<&'a str>);
		
		impl<'de, 'a> Visitor<'de> for ResourceUrlVisitor<'a>
		{
			type Value = ResourceUrl<'a>;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("a string")
			}
			
			#[inline(always)]
			fn visit_string<E: DeserializeError>(self, v: String) -> Result<Self::Value, E>
			{
				Ok(ResourceUrl::string(v))
			}
		}
		
		deserializer.deserialize_string(ResourceUrlVisitor(PhantomData))
	}
}

impl ResourceUrl<'static>
{
	#[inline(always)]
	pub(crate) fn get<'resources>(&self, resources: &'resources Resources) ->  Option<&'resources RefCell<Resource>>
	{
		resources.get(self)
	}
}

impl<'a> ResourceUrl<'a>
{
	#[inline(always)]
	pub(crate) fn string<S: Into<String>>(value: S) -> Self
	{
		ResourceUrl(Rc::new(Cow::Owned(value.into())))
	}
	
	#[inline(always)]
	pub(crate) fn str(value: &'a str) -> Self
	{
		ResourceUrl(Rc::new(Cow::Borrowed(value)))
	}
	
	#[inline(always)]
	pub(crate) fn rssUrl(iso_639_1_alpha_2_language_code: &str) -> Self
	{
		Self::string(format!("{}.rss.xml", iso_639_1_alpha_2_language_code))
	}
	
	#[inline(always)]
	pub(crate) fn siteMapUrl(iso_639_1_alpha_2_language_code: &str, index: usize) -> Self
	{
		Self::string(format!("{}.sitemap.{}.xml", index, iso_639_1_alpha_2_language_code))
	}
	
	#[inline(always)]
	pub(crate) fn siteMapIndexUrl(iso_639_1_alpha_2_language_code: &str, index: usize) -> Self
	{
		Self::string(format!("{}.sitemap-index.{}.xml", index, iso_639_1_alpha_2_language_code))
	}
	
	#[inline(always)]
	pub(crate) fn replaceFileNameExtension(&self, extension: &str) -> Self
	{
		match self.0.rfind('.')
		{
			None => Self::appendFileNameExtension(self.0.deref(), extension),
			Some(index) => Self::appendFileNameExtension(self.0.split_at(index).0, extension)
		}
	}
	
	#[inline(always)]
	pub(crate) fn withoutFileNameExtension(&'a self) -> Self
	{
		match self.0.rfind('.')
		{
			None => self.clone(),
			Some(index) => Self::str(self.0.split_at(index).0),
		}
	}
	
	#[inline(always)]
	pub(crate) fn toUrl(&self, baseUrl: Url) -> Result<Url, CordialError>
	{
		Ok(baseUrl.join(&self.0).context(format!("Invalid ResourceUrl '{}'", self))?)
	}
	
	#[inline(always)]
	pub(crate) fn leafUrl(&self) -> ResourceUrl
	{
		let mut leafPath = String::with_capacity(&self.0.len() + 1);
		leafPath.push_str(&self.0);
		leafPath.push('/');
		Self::string(leafPath)
	}
	
	#[inline(always)]
	pub(crate) fn widthUrl(&self, fileExtension: &'static str, width: u32) -> Self
	{
		let resourceRelativeUrlWithoutFileNameExtension = &self.0;
		
		let mut path = String::with_capacity(resourceRelativeUrlWithoutFileNameExtension.len() + 10);
		path.push_str(resourceRelativeUrlWithoutFileNameExtension.as_ref());
		path.push_str(&format!("-{}w", width));
		path.push_str(fileExtension);
		
		Self::string(path)
	}
	
	#[inline(always)]
	pub(crate) fn primaryUrl(&self, fileExtension: &'static str) -> Self
	{
		let resourceRelativeUrlWithoutFileNameExtension = &self.0;
		
		let mut path = String::with_capacity(resourceRelativeUrlWithoutFileNameExtension.len() + 10);
		path.push_str(resourceRelativeUrlWithoutFileNameExtension.as_ref());
		path.push_str(fileExtension);
		
		Self::string(path)
	}
	
	#[inline(always)]
	fn appendFileNameExtension(withoutFileNameExtension: &str, extension: &str) -> Self
	{
		let mut string = String::with_capacity(withoutFileNameExtension.len() + extension.len());
		string.push_str(withoutFileNameExtension.as_ref());
		string.push_str(extension);
		Self::string(string)
	}
}
