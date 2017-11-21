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
	pub(crate) fn execute(&self, arguments: &[u8], pluginData: &MarkdownPluginData, isForAmp: bool, ) -> Result<Vec<u8>, CordialError>
	{
		use self::MarkdownInlinePlugin::*;
		
		let mut arguments = parseQueryString(arguments);
		
		let string = match *self
		{
			image => Self::image(&mut arguments, pluginData, isForAmp)?,
		};
		Ok(string.into_bytes())
	}
	
	//noinspection SpellCheckingInspection
	fn image(arguments: &mut ParsedQueryString, pluginData: &MarkdownPluginData, isForAmp: bool) -> Result<String, CordialError>
	{
		let mut imageResourceUrl = None;
		let mut captionPosition = CaptionPosition::default();
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
				
				_ => return Err(CordialError::Configuration(format!("image inline plugin does not take the argument '{}'", name))),
			}
		}
		
		let image = match imageResourceUrl
		{
			None => return Err(CordialError::Configuration("image inline plugin resource can not be omitted".to_owned())),
			Some(url) => pluginData.image(&ResourceReference::primary_image(url))?,
		};
		
		// TODO: Support the image lightbox: https://ampbyexample.com/components/amp-image-lightbox/
		// - Need to add a hidden element to the page contents
		// on="tap:lightbox1" role="button" tabindex="0"    aria-describedby="imageDescription" (if not figcaption)
		
		
		// TODO: Animated GIFs need an alternative syntax of amp-anim
		// - Include <script async custom-element="amp-anim" src="https://cdn.ampproject.org/v0/amp-anim-0.1.js"></script> in the head of your page to use this component.
		// - Need to generate a GIF 'placeholder' image (ie from first frame)
		// - See https://ampbyexample.com/components/amp-anim/
		
		// TODO: Generate images suitable for Google VR View
		
		// NOTE: ampbyexample.org is a good example of a simple, responsive site although its load time is a little slow
		// Very clean, simple HTML, with header, footer, main, aside, article and section usage
		// h2 headings are the first element of a section
		// Looks like they are serving up 'amp' files on a non-amp URL.
		
		
		// TODO: Add to a list of resources that will need to be added to the page after rendering
		// TODO: Deserialize ResourceTag
		// TODO: For SVG, need to append / replace id, classes, title
		// TODO: embedding of SVG needs to be addressed
		// TODO: data-uris need to be addressed - is it really a resource-level property?
		// - they have lots of issues with google sitemaps, etc.
		// TODO: Make URLs relative
		
		use self::CaptionPosition::*;
		
		let node = if isForAmp
		{
			let ampImgNode = image.ampImgNode()?;
			
			match captionPosition
			{
				top =>
				{
					"figure"
					.with_child_element(image.figcaptionNode()?)
					.with_child_element(ampImgNode)
				}
				bottom =>
				{
					"figure"
					.with_child_element(ampImgNode)
					.with_child_element(image.figcaptionNode()?)
				}
				none => ampImgNode,
			}
		}
		else
		{
			let imgNode = image.imgNode()?;
			
			match captionPosition
			{
				top =>
				{
					"figure"
					.with_child_element(image.figcaptionNode()?)
					.with_child_element(imgNode)
				}
				bottom =>
				{
					"figure"
					.with_child_element(imgNode)
					.with_child_element(image.figcaptionNode()?)
				}
				none => imgNode,
			}
		};
		
		let mut rcDom = RcDom::default();
		node.attach_to_document_node(&mut rcDom);
		const html_head_and_body_tags_are_optional: bool = false;
		Ok(rcDom.minify_to_string(html_head_and_body_tags_are_optional))
	}
}
