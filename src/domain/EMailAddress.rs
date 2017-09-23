// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// NOTE: Almost equivalent to Rust email crate's ::email::Mailbox, but with a non-optional display name, and also equivalent to the email-format crate's ::email_format::rfc5322::types::NameAddr
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct EMailAddress
{
	full_name: FullName,
	email: String,
}

impl EMailAddress
{
	#[inline(always)]
	fn to_string(&self) -> String
	{
		format!("{} ({})", self.email, self.full_name)
	}
}