// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct BinaryBody(pub(crate) Vec<u8>);

impl AsRef<[u8]> for BinaryBody
{
	fn as_ref(&self) -> &[u8]
	{
		&self.0
	}
}

impl AsRef<Vec<u8>> for BinaryBody
{
	fn as_ref(&self) -> &Vec<u8>
	{
		self.deref()
	}
}

impl Deref for BinaryBody
{
	type Target = Vec<u8>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}
