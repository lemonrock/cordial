// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum PlotMarker
{
	circle,
	square,
}

impl Default for PlotMarker
{
	#[inline(always)]
	fn default() -> Self
	{
		PlotMarker::circle
	}
}

impl PlotMarker
{
	#[inline(always)]
	pub(crate) fn toMarker(&self) -> Marker
	{
		use self::PlotMarker::*;
		use self::Marker::*;
		
		match *self
		{
			circle => Circle,
			square => Square,
		}
	}
}
