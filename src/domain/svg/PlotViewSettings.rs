// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub(crate) struct PlotViewSettings
{
	x_minimum: f64,
	x_maximum: f64,
	
	y_minimum: f64,
	y_maximum: f64,
	
	#[serde(default = "PlotViewSettings::face_width_default")] face_width: f64,
	#[serde(default = "PlotViewSettings::face_height_default")] face_height: f64,
	
	#[serde(default = "PlotViewSettings::translate_x_default")] translate_x: u32,
	#[serde(default = "PlotViewSettings::translate_y_default")] translate_y: u32,
	
	#[serde(default)] plots: Vec<PlotKind>,
}

impl PlotViewSettings
{
	#[inline(always)]
	fn render(&self, document: &mut PlotSvgDocument)
	{
		// This ghastly code is because of a really poor API decision in plotlib: View.add(&self, &'a Representation) is not generic.
		
		let mut histogramsToMaintainLifetimesHack = Vec::with_capacity(self.plots.len());
		let mut scattersToMaintainLifetimesHack = Vec::with_capacity(self.plots.len());
		let mut view = View::new().x_range(self.x_minimum, self.x_maximum).y_range(self.y_minimum, self.y_maximum);
		
		for histogramPlotKind in self.plots.iter()
		{
			match *histogramPlotKind
			{
				PlotKind::Histogram { ref data, number_of_bins } =>
				{
					let histogram = Histogram::from_vec(&data, number_of_bins);
					histogramsToMaintainLifetimesHack.push(histogram);
				}
				_ => (),
			};
		}
		for histogram in histogramsToMaintainLifetimesHack.iter()
		{
			view = view.add(histogram);
		}
		
		for scatterPlotKind in self.plots.iter()
		{
			match *scatterPlotKind
			{
				PlotKind::Scatter { ref data, ref marker, ref color } =>
				{
					let style = Style::new().marker(marker.toMarker()).colour(color.toStringWithHashPrefix());
					scattersToMaintainLifetimesHack.push(Scatter::from_vec(&data).style(style));
				}
				_ => (),
			};
		}
		for scatter in scattersToMaintainLifetimesHack.iter()
		{
			view = view.add(scatter);
		}
		
		let gElement = view.to_svg(self.face_width, self.face_height).set("transform", format!("translate({}, {})", self.translate_x, self.translate_y));
		use ::svg::Node;
		document.append(gElement);
	}
	
	#[inline(always)]
	fn face_width_default() -> f64
	{
		500.0
	}
	
	#[inline(always)]
	fn face_height_default() -> f64
	{
		340.0
	}
	
	#[inline(always)]
	fn translate_x_default() -> u32
	{
		60
	}
	
	#[inline(always)]
	fn translate_y_default() -> u32
	{
		360
	}
}
