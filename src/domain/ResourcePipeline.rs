// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) enum ResourcePipeline
{
	browser_config,
	css,
	font,
	gif_animation,
	html,
	raster_image,
	raw,
	svg,
	web_app_manifest,
	video,
}

impl Default for ResourcePipeline
{
	#[inline(always)]
	fn default() -> Self
	{
		ResourcePipeline::raw
	}
}
