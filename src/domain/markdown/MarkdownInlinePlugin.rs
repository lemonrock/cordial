// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum MarkdownInlinePlugin
{
	image,
}

impl MarkdownInlinePlugin
{
	#[inline(always)]
	pub(crate) fn registerAllPlugins() -> HashMap<Vec<u8>, MarkdownInlinePlugin>
	{
		use self::MarkdownInlinePlugin::*;
		
		hashmap!
		{
			b"image".to_vec() => image,
		}
	}
	
	#[inline(always)]
	pub(crate) fn execute(&self, arguments: &[u8], nodesForOtherPlacesInHtml: &mut NodesForOtherPlacesInHtml, markdownPluginData: &MarkdownPluginData, isForAmp: bool) -> Result<MarkdownPluginResult, CordialError>
	{
		use self::MarkdownInlinePlugin::*;
		
		let mut arguments = parseQueryString(arguments);
		
		match *self
		{
			image => Self::image(&mut arguments, nodesForOtherPlacesInHtml, markdownPluginData, isForAmp),
		}
	}
	
	//noinspection SpellCheckingInspection
	fn image(arguments: &mut ParsedQueryString, nodesForOtherPlacesInHtml: &mut NodesForOtherPlacesInHtml, markdownPluginData: &MarkdownPluginData, isForAmp: bool) -> Result<MarkdownPluginResult, CordialError>
	{
		let mut imageResourceUrl = None;
		let mut captionPosition = CaptionPosition::default();
		let mut lightboxId = None;
		let mut displayAmpLoadingIndicator = true;
		for (name, value) in arguments
		{
			match name.deref()
			{
				"image" =>
				{
					imageResourceUrl = Some(ResourceUrl(Rc::new(value.to_string())))
				}
				
				"caption" =>
				{
					captionPosition = CaptionPosition::parse(value.deref())?
				},
				
				"lightbox_id" if isForAmp =>
				{
					lightboxId = match value.deref()
					{
						"" => return Err(CordialError::Configuration("Empty is not a valid value for a lightbox_id".to_owned())),
						value @ _ => Some(value.to_owned()),
					};
				}
				
				"hide_loading_indicator" if isForAmp =>
				{
					displayAmpLoadingIndicator = match value.deref()
					{
						"" | "n" => false,
						"n" => true,
						_ => return Err(CordialError::Configuration("Any value other than empty or y or n is not a valid value for a hide_loading_indicator".to_owned())),
					};
				}
				
				_ => return Err(CordialError::Configuration(format!("image inline plugin does not take the argument '{}'", name))),
			}
		}
		
		let image = match imageResourceUrl
		{
			None => return Err(CordialError::Configuration("image inline plugin resource can not be omitted".to_owned())),
			Some(imageResourceUrl) => markdownPluginData.image(imageResourceUrl)?,
		};
		
		use self::CaptionPosition::*;
		
		let imageNodeToWrapWithFigure = if isForAmp
		{
			if image.isAnimated()
			{
				nodesForOtherPlacesInHtml.ampScript("amp-anim", "https://cdn.ampproject.org/v0/amp-anim-0.1.js");
				image.ampAnimNode(displayAmpLoadingIndicator)?
			}
			else
			{
				if let Some(ref lightboxId) = lightboxId
				{
					nodesForOtherPlacesInHtml.ampScript("amp-image-lightbox", "https://cdn.ampproject.org/v0/amp-image-lightbox-0.1.js");
					nodesForOtherPlacesInHtml.hiddenBody(format!("amp-image-lightbox-{}", &lightboxId), "amp-image-lightbox".with_id_attribute(lightboxId).with_attribute(AmpLayout::nodisplay.toAttribute()) )
				}
				image.ampImgNode(image.isAnimated(), lightboxId, displayAmpLoadingIndicator)?
			}
		}
		else
		{
			image.imgNode()?
		};
		
		let figureNode = match captionPosition
		{
			top =>
			{
				"figure"
				.with_child_element(image.figcaptionNode()?)
				.with_child_element(imageNodeToWrapWithFigure)
			}
			bottom =>
			{
				"figure"
				.with_child_element(imageNodeToWrapWithFigure)
				.with_child_element(image.figcaptionNode()?)
			}
			none => imageNodeToWrapWithFigure,
		};
		
		MarkdownPluginResult::ok(vec![figureNode])
	}
}
