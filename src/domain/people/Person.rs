// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Person
{
	#[serde(default = "Person::full_name_default")] pub(crate) full_name: FullName,
	#[serde(default)] pub(crate) url: Option<ResourceUrl>,
}

impl Default for Person
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			full_name: Self::full_name_default(),
			url: None,
		}
	}
}

impl Person
{
	#[inline(always)]
	fn full_name_default() -> FullName
	{
		Rc::new("webmaster@example.com".to_owned())
	}
}
