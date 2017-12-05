// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait XmlNameExt
{
	#[inline(always)]
	fn xml_local_name<'a>(&'a self) -> Name<'a>;
	
	#[inline(always)]
	fn prefixes_xml_name<'a>(&'a self, name: &'a str) -> Name<'a>;
}

impl XmlNameExt for str
{
	#[inline(always)]
	fn xml_local_name<'a>(&'a self) -> Name<'a>
	{
		Name::local(self)
	}
	
	#[inline(always)]
	fn prefixes_xml_name<'a>(&'a self, name: &'a str) -> Name<'a>
	{
		Name::prefixed(name, self)
	}
}
