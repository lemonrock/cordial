// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum EngiffenQuantizer
{
	naive,
	neu_quant(#[serde(default = "EngiffenQuantizer::neu_quant_default")] u32),
}

impl Default for EngiffenQuantizer
{
	#[inline(always)]
	fn default() -> Self
	{
		EngiffenQuantizer::neu_quant(Self::neu_quant_default())
	}
}

impl EngiffenQuantizer
{
	#[inline(always)]
	fn toQuantizer(&self) -> Quantizer
	{
		use self::EngiffenQuantizer::*;
		use ::engiffen::Quantizer::*;
		
		match *self
		{
			naive => Naive,
			neu_quant(value) =>
			{
				let value = if value == 0
				{
					1
				}
				else
				{
					value
				};
				NeuQuant(value)
			}
		}
	}
	
	#[inline(always)]
	fn neu_quant_default() -> u32
	{
		1
	}
}
