// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) enum ResourcePipeline
{
	css(CssPipeline),
	
	font(FontPipeline),
	
	gif_animation(GifAnimationPipeline),
	
	html(HtmlPipeline),
	
	raster_image(RasterImagePipeline),
	
	raw(RawPipeline),
	
	svg(SvgPipeline),
}

impl Default for ResourcePipeline
{
	#[inline(always)]
	fn default() -> Self
	{
		ResourcePipeline::raw(Default::default())
	}
}

impl ResourcePipeline
{
	#[inline(always)]
	pub(crate) fn processingPriority(&self) -> ProcessingPriority
	{
		use self::ResourcePipeline::*;
		match *self
		{
			css(ref pipeline) => pipeline.processingPriority(),
			font(ref pipeline) => pipeline.processingPriority(),
			gif_animation(ref pipeline) => pipeline.processingPriority(),
			html(ref pipeline) => pipeline.processingPriority(),
			raster_image(ref pipeline) => pipeline.processingPriority(),
			raw(ref pipeline) => pipeline.processingPriority(),
			svg(ref pipeline) => pipeline.processingPriority(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		use self::ResourcePipeline::*;
		match *self
		{
			css(ref pipeline) => pipeline.resourceInputContentFileNamesWithExtension(resourceInputName),
			font(ref pipeline) => pipeline.resourceInputContentFileNamesWithExtension(resourceInputName),
			gif_animation(ref pipeline) => pipeline.resourceInputContentFileNamesWithExtension(resourceInputName),
			html(ref pipeline) => pipeline.resourceInputContentFileNamesWithExtension(resourceInputName),
			raster_image(ref pipeline) => pipeline.resourceInputContentFileNamesWithExtension(resourceInputName),
			raw(ref pipeline) => pipeline.resourceInputContentFileNamesWithExtension(resourceInputName),
			svg(ref pipeline) => pipeline.resourceInputContentFileNamesWithExtension(resourceInputName),
		}
	}
	
	#[inline(always)]
	pub(crate) fn is<'a>(&self) -> (bool, bool)
	{
		use self::ResourcePipeline::*;
		match *self
		{
			css(ref pipeline) => pipeline.is(),
			font(ref pipeline) => pipeline.is(),
			gif_animation(ref pipeline) => pipeline.is(),
			html(ref pipeline) => pipeline.is(),
			raster_image(ref pipeline) => pipeline.is(),
			raw(ref pipeline) => pipeline.is(),
			svg(ref pipeline) => pipeline.is(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn execute(&mut self, inputContentFilePath: &Path, resourceRelativeUrl: &str, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: &LanguageData, ifLanguageAwareLanguageData: Option<&LanguageData>, configuration: &Configuration, siteMapWebPages: &mut Vec<SiteMapWebPage>, rssItems: &mut Vec<RssItem>) -> Result<Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		use self::ResourcePipeline::*;
		match *self
		{
			css(ref mut pipeline) => pipeline.execute(inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
			font(ref mut pipeline) => pipeline.execute(inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
			gif_animation(ref mut pipeline) => pipeline.execute(inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
			html(ref mut pipeline) => pipeline.execute(inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
			raster_image(ref mut pipeline) => pipeline.execute(inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
			raw(ref mut pipeline) => pipeline.execute(inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
			svg(ref mut pipeline) => pipeline.execute(inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
		}
	}
}
