// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub(crate) struct PlotSettings
{
	#[serde(default)] viewbox_x_minimum: u32,
	#[serde(default)] viewbox_y_minimum: u32,
	#[serde(default = "PlotSettings::viewbox_x_maximum_default")] viewbox_x_maximum: u32,
	#[serde(default = "PlotSettings::viewbox_y_maximum_default")] viewbox_y_maximum: u32,
	#[serde(default)] views: Vec<PlotViewSettings>,
}

impl Default for PlotSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			viewbox_x_minimum: 0,
			viewbox_y_minimum: 0,
			viewbox_x_maximum: Self::viewbox_x_maximum_default(),
			viewbox_y_maximum: Self::viewbox_y_maximum_default(),
			views: Default::default(),
		}
	}
}

impl PlotSettings
{
	#[inline(always)]
	pub(crate) fn svgString(&self) -> Result<String, CordialError>
	{
		let mut document = PlotSvgDocument::new().set("viewBox", (self.viewbox_x_minimum, self.viewbox_y_minimum, self.viewbox_x_maximum, self.viewbox_y_maximum));
		for view in self.views.iter()
		{
			view.render(&mut document)
		}
		Ok(format!("{}", document))
	}
	
	fn viewbox_x_maximum_default() -> u32
	{
		600
	}
	
	#[inline(always)]
	fn viewbox_y_maximum_default() -> u32
	{
		400
	}
}
