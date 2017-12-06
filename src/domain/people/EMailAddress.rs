// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// NOTE: Almost equivalent to Rust email crate's ::email::Mailbox, but with a non-optional display name, and also equivalent to the email-format crate's ::email_format::rfc5322::types::NameAddr
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct EMailAddress
{
	#[serde(default = "EMailAddress::full_name_default")] pub(crate) full_name: FullName,
	#[serde(default = "EMailAddress::email_default")] pub(crate) email: Rc<String>,
}

impl Default for EMailAddress
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			full_name: Self::full_name_default(),
			email: Self::email_default(),
		}
	}
}

impl EMailAddress
{
	#[inline(always)]
	pub(crate) fn to_string(&self) -> String
	{
		format!("{} ({})", self.email, self.full_name)
	}
	
	#[inline(always)]
	fn full_name_default() -> FullName
	{
		Rc::new("webmaster@example.com".to_owned())
	}
	
	#[inline(always)]
	fn email_default() -> Rc<String>
	{
		Rc::new("Webmaster".to_owned())
	}
}
