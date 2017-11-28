// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.



pub(crate) trait UnattachedNodeHelperExt
{
	#[inline(always)]
	fn empty_attribute(&self) -> Attribute;
	
	#[inline(always)]
	fn string_attribute(&self, value: String) -> Attribute
	{
		self.str_attribute(&value)
	}
	
	#[inline(always)]
	fn str_attribute(&self, value: &str) -> Attribute;
	
	#[inline(always)]
	fn u16_attribute(&self, value: u16) -> Attribute;
	
	#[inline(always)]
	fn u32_attribute(&self, value: u32) -> Attribute;
	
	#[inline(always)]
	fn space_separated_attribute<S: Deref<Target=str>>(&self, values: &[S]) -> Attribute
	{
		self.separated_attribute(values, ' ')
	}
	
	#[inline(always)]
	fn separated_attribute<S: Deref<Target=str>>(&self, values: &[S], separator: char) -> Attribute
	{
		let mut afterFirst = false;
		let mut attributeString = String::new();
		for value in values.iter()
		{
			if afterFirst
			{
				attributeString.push(separator);
			}
			else
			{
				afterFirst = false;
			}
			attributeString.push_str(value);
		}
		self.string_attribute(attributeString)
	}
	
	#[inline(always)]
	fn with_id_attribute(&self, id: &str) -> UnattachedNode;
	
	#[inline(always)]
	fn with_charset_attribute(&self, charset: &str) -> UnattachedNode;
	
	#[inline(always)]
	fn with_name_attribute(&self, charset: &str) -> UnattachedNode;
	
	#[inline(always)]
	fn with_property_attribute(&self, charset: &str) -> UnattachedNode;
	
	#[inline(always)]
	fn with_http_equiv_attribute(&self, charset: &str) -> UnattachedNode;
	
	#[inline(always)]
	fn with_empty_attribute(&self, name: &str) -> UnattachedNode;
	
	#[inline(always)]
	fn with_async_attribute(&self) -> UnattachedNode
	{
		self.with_empty_attribute("async")
	}
	
	#[inline(always)]
	fn with_amp_boilerplate_attribute(&self) -> UnattachedNode
	{
		self.with_empty_attribute("amp-boilerplate")
	}
	
	#[inline(always)]
	fn with_attributes(&self, attributes: Vec<Attribute>) -> UnattachedNode;
	
	#[inline(always)]
	fn with_child_element(&self, child: UnattachedNode) -> UnattachedNode;
	
	#[inline(always)]
	fn with_child_text<S: Into<String>>(&self, text: S) -> UnattachedNode;
	
	#[inline(always)]
	fn with_class<S: Deref<Target=str>>(&self, class: S) -> UnattachedNode;
	
	#[inline(always)]
	fn with_classes<S: Deref<Target=str>>(&self, classes: &[S]) -> UnattachedNode;
	
	#[inline(always)]
	fn local_name(&self) -> LocalName;
	
	#[inline(always)]
	fn amp_script(&self, url: &str) -> UnattachedNode;
}

impl UnattachedNodeHelperExt for str
{
	#[inline(always)]
	fn empty_attribute(&self) -> Attribute
	{
		self.local_name().empty_attribute()
	}
	
	#[inline(always)]
	fn str_attribute(&self, value: &str) -> Attribute
	{
		self.local_name().attribute(value)
	}
	
	#[inline(always)]
	fn u16_attribute(&self, value: u16) -> Attribute
	{
		self.string_attribute(format!("{}", value))
	}
	
	#[inline(always)]
	fn u32_attribute(&self, value: u32) -> Attribute
	{
		self.string_attribute(format!("{}", value))
	}
	
	#[inline(always)]
	fn with_id_attribute(&self, id: &str) -> UnattachedNode
	{
		self.local_name().with_attributes(vec!["id".str_attribute(id)])
	}
	
	#[inline(always)]
	fn with_charset_attribute(&self, charset: &str) -> UnattachedNode
	{
		self.local_name().with_attributes(vec!["charset".str_attribute(charset)])
	}
	
	#[inline(always)]
	fn with_name_attribute(&self, name: &str) -> UnattachedNode
	{
		self.local_name().with_attributes(vec!["name".str_attribute(name)])
	}
	
	#[inline(always)]
	fn with_property_attribute(&self, name: &str) -> UnattachedNode
	{
		self.local_name().with_attributes(vec!["property".str_attribute(name)])
	}
	
	#[inline(always)]
	fn with_http_equiv_attribute(&self, name: &str) -> UnattachedNode
	{
		self.local_name().with_attributes(vec!["http-equiv".str_attribute(name)])
	}
	
	#[inline(always)]
	fn with_empty_attribute(&self, name: &str) -> UnattachedNode
	{
		self.local_name().with_attributes(vec![name.empty_attribute()])
	}
	
	#[inline(always)]
	fn with_attributes(&self, attributes: Vec<Attribute>) -> UnattachedNode
	{
		self.local_name().with_attributes(attributes)
	}
	
	#[inline(always)]
	fn with_child_element(&self, child: UnattachedNode) -> UnattachedNode
	{
		let localName = self.local_name();
		UnattachedNode::from(localName).with_child_element(child)
	}
	
	#[inline(always)]
	fn with_child_text<S: Into<String>>(&self, text: S) -> UnattachedNode
	{
		let localName = self.local_name();
		UnattachedNode::from(localName).with_child_text(text)
	}
	
	#[inline(always)]
	fn with_class<S: Deref<Target=str>>(&self, class: S) -> UnattachedNode
	{
		self.local_name().with_classes(&[class])
	}
	
	#[inline(always)]
	fn with_classes<S: Deref<Target=str>>(&self, classes: &[S]) -> UnattachedNode
	{
		self.local_name().with_classes(classes)
	}
	
	#[inline(always)]
	fn local_name(&self) -> LocalName
	{
		LocalName::from(self)
	}
	
	#[inline(always)]
	fn amp_script(&self, url: &str) -> UnattachedNode
	{
		"script"
			.with_async_attribute()
			.with_attribute("custom-element".str_attribute(self))
			.with_attribute("src".str_attribute(url))
	}
}
