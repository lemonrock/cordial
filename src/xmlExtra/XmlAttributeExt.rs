// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait XmlAttributeExt
{
	#[inline(always)]
	fn xml_str_attribute<'a>(&'a self, value: &'a str) -> Attribute<'a>;
	
	#[inline(always)]
	fn xml_string_attribute(&self, value: String) -> OwnedAttribute
	{
		self.xml_str_attribute(&value).to_owned()
	}
	
	#[inline(always)]
	fn xml_u32_attribute(&self, value: u32) -> OwnedAttribute
	{
		self.xml_string_attribute(format!("{}", value))
	}
	
	#[inline(always)]
	fn xml_u64_attribute(&self, value: u64) -> OwnedAttribute
	{
		self.xml_string_attribute(format!("{}", value))
	}
	
	#[inline(always)]
	fn xml_language_attribute<'a>(&'a self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Attribute<'a>
	{
		self.xml_str_attribute(iso639Dash1Alpha2Language.to_iso_639_1_alpha_2_language_code())
	}
	
	#[inline(always)]
	fn xml_url_attribute<'a>(&'a self, value: &'a Url) -> Attribute<'a>
	{
		self.xml_str_attribute(value.as_ref())
	}
	
	#[inline(always)]
	fn xml_url_from_UrlData_attribute<'a>(&'a self, value: &'a UrlData) -> Attribute<'a>
	{
		self.xml_str_attribute(value.url_str())
	}
}

impl XmlAttributeExt for str
{
	#[inline(always)]
	fn xml_str_attribute<'a>(&'a self, value: &'a str) -> Attribute<'a>
	{
		Attribute::new(self.xml_local_name(), value)
	}
}
