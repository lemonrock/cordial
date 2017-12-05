// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct HexadecimalColor(pub(crate) [u8; 3]);

impl HexadecimalColor
{
	#[inline(always)]
	fn toStringWithoutHashPrefix(&self) -> String
	{
		format!("{:02X}{:02X}{:02X}", self.0[0], self.0[1], self.0[2])
	}
	
	#[inline(always)]
	fn toStringWithHashPrefix(&self) -> String
	{
		format!("#{:02X}{:02X}{:02X}", self.0[0], self.0[1], self.0[2])
	}
}
