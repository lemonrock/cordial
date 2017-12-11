// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub(crate) enum PlotKind
{
	Histogram
	{
		#[serde(default)] data: Vec<f64>,
		#[serde(default = "PlotKind::number_of_bins_default")] number_of_bins: u32,
	},
	
	Scatter
	{
		#[serde(default)] data: Vec<(f64, f64)>,
		#[serde(default)] marker: PlotMarker,
		#[serde(default)] color: HexadecimalColor,
	},
}

impl PlotKind
{
	#[inline(always)]
	fn number_of_bins_default() -> u32
	{
		8
	}
}

impl Default for PlotKind
{
	#[inline(always)]
	fn default() -> Self
	{
		PlotKind::Histogram
		{
			data: Default::default(),
			number_of_bins: Self::number_of_bins_default(),
		}
	}
}
