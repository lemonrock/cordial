// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum MarkdownInlinePlugin
{
	//app_banner,
	image,
	audio,
	video,
	shortcode,
}

impl MarkdownInlinePlugin
{
	#[inline(always)]
	pub(crate) fn registerAllPlugins() -> HashMap<Vec<u8>, MarkdownInlinePlugin>
	{
		use self::MarkdownInlinePlugin::*;
		
		hashmap!
		{
			//b"app_banner".to_vec() => app_banner,
			b"image".to_vec() => image,
			b"audio".to_vec() => audio,
			b"video".to_vec() => video,
			b"shortcode".to_vec() => shortcode,
		}
	}
	
	#[inline(always)]
	pub(crate) fn execute(&self, arguments: &[u8], nodesForOtherPlacesInHtml: &mut NodesForOtherPlacesInHtml, markdownPluginData: &MarkdownPluginData, isForAmp: bool) -> Result<MarkdownPluginResult, CordialError>
	{
		use self::MarkdownInlinePlugin::*;
		
		match *self
		{
			//app_banner => Self::image(&mut arguments, nodesForOtherPlacesInHtml, markdownPluginData, isForAmp),
			image => Self::image(&mut parseQueryString(arguments), nodesForOtherPlacesInHtml, markdownPluginData, isForAmp),
			audio => Self::audio(&mut parseQueryString(arguments), nodesForOtherPlacesInHtml, markdownPluginData, isForAmp),
			video => Self::video(&mut parseQueryString(arguments), nodesForOtherPlacesInHtml, markdownPluginData, isForAmp),
			shortcode => Self::shortcode(arguments, nodesForOtherPlacesInHtml, markdownPluginData, isForAmp),
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
				"url" =>
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
						"y" => true,
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
				image.ampImgNode(image.isAnimated(), lightboxId, displayAmpLoadingIndicator, AmpLayout::responsive)?
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
	
	fn audio(arguments: &mut ParsedQueryString, nodesForOtherPlacesInHtml: &mut NodesForOtherPlacesInHtml, markdownPluginData: &MarkdownPluginData, isForAmp: bool) -> Result<MarkdownPluginResult, CordialError>
	{
		let mut audioResourceUrl = None;
		for (name, value) in arguments
		{
			match name.deref()
			{
				"url" =>
				{
					audioResourceUrl = Some(ResourceUrl(Rc::new(value.to_string())))
				}
				
				_ => return Err(CordialError::Configuration(format!("audio inline plugin does not take the argument '{}'", name))),
			}
		}
		
		let audioNode = match audioResourceUrl
		{
			None => return Err(CordialError::Configuration("audio inline plugin resource can not be omitted".to_owned())),
			Some(audioResourceUrl) => markdownPluginData.audioNode(audioResourceUrl, isForAmp)?,
		};
		
		if isForAmp
		{
			nodesForOtherPlacesInHtml.ampScript("amp-audio", "https://cdn.ampproject.org/v0/amp-audio-0.1.js")
		}
		
		MarkdownPluginResult::ok(vec![audioNode])
	}
	
	fn video(arguments: &mut ParsedQueryString, nodesForOtherPlacesInHtml: &mut NodesForOtherPlacesInHtml, markdownPluginData: &MarkdownPluginData, isForAmp: bool) -> Result<MarkdownPluginResult, CordialError>
	{
		let mut videoResourceUrl = None;
		for (name, value) in arguments
		{
			match name.deref()
			{
				"url" =>
				{
					videoResourceUrl = Some(ResourceUrl(Rc::new(value.to_string())))
				}
				
				_ => return Err(CordialError::Configuration(format!("video inline plugin does not take the argument '{}'", name))),
			}
		}
		
		let videoNode = match videoResourceUrl
		{
			None => return Err(CordialError::Configuration("video inline plugin resource can not be omitted".to_owned())),
			Some(videoResourceUrl) => markdownPluginData.videoNode(videoResourceUrl, isForAmp)?,
		};
		
		if isForAmp
		{
			nodesForOtherPlacesInHtml.ampScript("amp-video", "https://cdn.ampproject.org/v0/amp-video-0.1.js")
		}
		
		MarkdownPluginResult::ok(vec![videoNode])
	}
	
	fn shortcode(arguments: &[u8], nodesForOtherPlacesInHtml: &mut NodesForOtherPlacesInHtml, markdownPluginData: &MarkdownPluginData, isForAmp: bool) -> Result<MarkdownPluginResult, CordialError>
	{
		let length = arguments.len();
		let mut shortCodeIndex = length;
		for index in 0 .. length
		{
			if arguments[index] == b' '
			{
				shortCodeIndex = index;
				break;
			}
		}
		
		let shortCodeName = match from_utf8(&arguments[0..shortCodeIndex])
		{
			Err(_) => return Err(CordialError::Configuration("shortcode name was not UTF-8 encoded".to_owned())),
			Ok(shortCode) => shortCode,
		};
		
		if shortCodeName.is_empty()
		{
			return Err(CordialError::Configuration("shortcode name was empty".to_owned()))
		}
		
		let arguments = if shortCodeIndex != length
		{
			Some(parseQueryString(&arguments[shortCodeIndex + 1..]))
		}
		else
		{
			None
		};
		
		let bytes = LuaShortCodeHelper::newForMarkdownPlugin(markdownPluginData.configuration).callFromMarkdownInlinePlugin(shortCodeName, arguments)?;
		
		MarkdownPluginResult::fromBytes(bytes)
	}
}
