// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ResourceUrl(Rc<String>);

impl Default for ResourceUrl
{
	#[inline(always)]
	fn default() -> Self
	{
		ResourceUrl::string("/")
	}
}

impl Display for ResourceUrl
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.0.fmt(f)
	}
}

impl<'de> Deserialize<'de> for ResourceUrl
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct ResourceUrlVisitor;
		
		impl<'de> Visitor<'de> for ResourceUrlVisitor
		{
			type Value = ResourceUrl;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("a string")
			}
			
			#[inline(always)]
			fn visit_str<E: DeserializeError>(self, v: &str) -> Result<Self::Value, E>
			{
				Ok(ResourceUrl::string(v))
			}
			
			#[inline(always)]
			fn visit_string<E: DeserializeError>(self, v: String) -> Result<Self::Value, E>
			{
				Ok(ResourceUrl::string(v))
			}
		}
		
		deserializer.deserialize_string(ResourceUrlVisitor)
	}
}

impl Serialize for ResourceUrl
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_str(&self.0)
	}
}

impl ResourceUrl
{
	#[inline(always)]
	pub(crate) fn resource<'resources>(&self, resources: &'resources Resources) -> Option<&'resources RefCell<Resource>>
	{
		resources.get(self)
	}
	
	#[inline(always)]
	pub(crate) fn resourceMandatory<'resources>(&self, resources: &'resources Resources) -> Result<Ref<'resources, Resource>, CordialError>
	{
		let resourceRefCell = self.resource(resources).ok_or_else(|| CordialError::Configuration(format!("Could not obtain resource '{:?}'", self)))?;
		let borrowedResource = resourceRefCell.try_borrow()?;
		Ok(borrowedResource)
	}
	
	#[inline(always)]
	pub(crate) fn validateResourceExists<'resources>(&self, resources: &'resources Resources) -> Result<(), CordialError>
	{
		self.resourceMandatory(resources).map(|_| ())
	}
	
	#[inline(always)]
	pub(crate) fn findUrlForFacebookOpenGraph<'resources>(&self, resources: &'resources Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, facebookOpenGraphTypeDiscriminant: FacebookOpenGraphTypeDiscriminant) -> Result<Rc<Url>, CordialError>
	{
		const FacebookUrlTag: ResourceTag = ResourceTag::default;
		
		let resource = self.resourceMandatory(resources)?;
		resource.findUrlForFacebookOpenGraph(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language), &FacebookUrlTag, facebookOpenGraphTypeDiscriminant).map(|url| url.clone())
	}
	
	#[inline(always)]
	pub(crate) fn string<S: Into<String>>(value: S) -> Self
	{
		ResourceUrl(Rc::new(value.into()))
	}
	
	#[inline(always)]
	pub(crate) fn rssUrl(rssChannelName: &Rc<RssChannelName>, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Self
	{
		Self::string(format!("{}.{}.rss", rssChannelName, iso639Dash1Alpha2Language))
	}
	
	#[inline(always)]
	pub(crate) fn siteMapUrl(iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, index: usize) -> Self
	{
		Self::string(format!("{}.{}.sitemap.xml", index, iso639Dash1Alpha2Language))
	}
	
	#[inline(always)]
	pub(crate) fn siteMapIndexUrl(iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, index: usize) -> Self
	{
		Self::string(format!("{}.{}.sitemap-index.xml", index, iso639Dash1Alpha2Language))
	}
	
	#[inline(always)]
	pub(crate) fn widthUrl(resourceRelativeUrlWithoutFileNameExtension: &str, fileExtension: &'static str, languageData: &LanguageData, width: u32) -> Result<Url, CordialError>
	{
		#[inline(always)]
		fn widthUrl(resourceRelativeUrlWithoutFileNameExtension: &str, fileExtension: &'static str, width: u32) -> ResourceUrl
		{
			let mut path = String::with_capacity(resourceRelativeUrlWithoutFileNameExtension.len() + 10);
			path.push_str(resourceRelativeUrlWithoutFileNameExtension.as_ref());
			path.push_str(&format!("-{}w", width));
			path.push_str(fileExtension);
			
			ResourceUrl::string(path)
		}
		
		widthUrl(resourceRelativeUrlWithoutFileNameExtension, fileExtension, width).url(languageData)
	}
	
	#[inline(always)]
	pub(crate) fn primaryUrl(resourceRelativeUrlWithoutFileNameExtension: &str, fileExtension: &'static str, languageData: &LanguageData) -> Result<Url, CordialError>
	{
		#[inline(always)]
		fn primaryUrl(resourceRelativeUrlWithoutFileNameExtension: &str, fileExtension: &'static str) -> ResourceUrl
		{
			let mut path = String::with_capacity(resourceRelativeUrlWithoutFileNameExtension.len() + 10);
			path.push_str(resourceRelativeUrlWithoutFileNameExtension.as_ref());
			path.push_str(fileExtension);
			
			ResourceUrl::string(path)
		}
		
		primaryUrl(resourceRelativeUrlWithoutFileNameExtension, fileExtension).url(languageData)
	}
	
	#[inline(always)]
	pub(crate) fn leafUrl(&self) -> Self
	{
		let mut leafPath = String::with_capacity(&self.0.len() + 1);
		leafPath.push_str(&self.0);
		leafPath.push('/');
		Self::string(leafPath)
	}
	
	#[inline(always)]
	pub(crate) fn leaf_url(&self, languageData: &LanguageData) -> Result<Url, CordialError>
	{
		self._leaf_url(languageData, false)
	}
	
	#[inline(always)]
	pub(crate) fn url(&self, languageData: &LanguageData) -> Result<Url, CordialError>
	{
		self._url(languageData, false)
	}
	
	#[inline(always)]
	pub(crate) fn amp_leaf_url(&self, languageData: &LanguageData) -> Result<Url, CordialError>
	{
		self._leaf_url(languageData, true)
	}
	
	#[inline(always)]
	pub(crate) fn amp_url(&self, languageData: &LanguageData) -> Result<Url, CordialError>
	{
		self._url(languageData, true)
	}
	
	#[inline(always)]
	pub(crate) fn toUrl(&self, baseUrl: Url) -> Result<Url, CordialError>
	{
		Ok(baseUrl.join(&self.0).context(format!("Invalid ResourceUrl '{}'", self))?)
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
	pub(crate) fn withoutFileNameExtension(&self) -> &str
	{
		match self.0.rfind('.')
		{
			None => &self.0,
			Some(index) => self.0.split_at(index).0,
		}
	}
	
	
	#[inline(always)]
	fn _leaf_url(&self, languageData: &LanguageData, isForAmp: bool) -> Result<Url, CordialError>
	{
		self.leafUrl()._url(languageData, isForAmp)
	}
	
	#[inline(always)]
	fn _url(&self, languageData: &LanguageData, isForAmp: bool) -> Result<Url, CordialError>
	{
		self.toUrl(languageData.baseUrl(isForAmp)?)
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
