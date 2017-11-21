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
		enum CaptionPosition
		{
			top,
			bottom,
			none,
		}
		
		impl Default for CaptionPosition
		{
			#[inline(always)]
			fn default() -> Self
			{
				CaptionPosition::bottom
			}
		}
		
		impl CaptionPosition
		{
			#[inline(always)]
			fn parse(value: &str) -> Result<Self, CordialError>
			{
				match value
				{
					"top" => Ok(CaptionPosition::top),
					"bottom" => Ok(CaptionPosition::bottom),
					"none" => Ok(CaptionPosition::none),
					_ => Err(CordialError::Configuration(format!("The caption position '{}' is not valid", value))),
				}
			}
		}
		
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
		
		let (imageResource, imageMetaData, imageUrlData, imageAbstract) = match imageResourceUrl
		{
			None => return Err(CordialError::Configuration("image inline plugin resource can not be omitted".to_owned())),
			Some(url) => pluginData.image(&ResourceReference::primary_image(url))?,
		};
		
		// TODO: Translations
		// TODO: Probably better done as a CSS content atttribute
		let fallbackText_TO_TRANSLATE = "Unfortunately, this content is unavailable at this time.";
		
		
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
		
		let imgNode = Self::imgNode(&imageUrlData, &imageAbstract, &imageMetaData, &imageResource, &pluginData)?;
		
		let node = if isForAmp
		{
			let ampImgNode = Self::ampImgNode(&imageUrlData, &imageAbstract, &imageMetaData, &imageResource, &pluginData, imgNode, fallbackText_TO_TRANSLATE)?;
			
			match captionPosition
			{
				CaptionPosition::top =>
				{
					"figure"
					.with_child_element(Self::figcaptionNode(&imageAbstract, &imageMetaData, &pluginData)?)
					.with_child_element(ampImgNode)
				}
				CaptionPosition::bottom =>
				{
					"figure"
					.with_child_element(ampImgNode)
					.with_child_element(Self::figcaptionNode(&imageAbstract, &imageMetaData, &pluginData)?)
				}
				CaptionPosition::none => ampImgNode,
			}
		}
		else
		{
			match captionPosition
			{
				CaptionPosition::top =>
				{
					"figure"
					.with_child_element(Self::figcaptionNode(imageAbstract, imageMetaData, pluginData)?)
					.with_child_element(imgNode)
				}
				CaptionPosition::bottom =>
				{
					"figure"
					.with_child_element(imgNode)
					.with_child_element(Self::figcaptionNode(imageAbstract, imageMetaData, pluginData)?)
				}
				CaptionPosition::none => imgNode,
			}
		};
		
		let mut rcDom = RcDom::default();
		node.attach_to_document_node(&mut rcDom);
		const html_head_and_body_tags_are_optional: bool = false;
		Ok(rcDom.minify_to_string(html_head_and_body_tags_are_optional))
	}
	
	
	/*
	<figure>
		<amp-img
		layout="responsive" attribution="Joe Blogs"
		alt="AMP" width="475" height="268" src="/img/amp.jpg" srcset="/img/amp.jpg 1080w, /img/amp-900.jpg 900w, /img/amp-800.jpg 800w,/img/amp-700.jpg 700w, /img/amp-600.jpg 600w, /img/amp-500.jpg 500w, /img/amp-400.jpg 400w,/img/amp-300.jpg 300w, /img/amp-200.jpg 200w, /img/amp-100.jpg 100w"
		>
			<div fallback>HTML displayed if offline</div>
			<noscript>
				<!-- Regular img attribute -->
				<img alt="AMP" width="475" height="268" src="/img/amp.jpg" srcset="/img/amp.jpg 1080w, /img/amp-900.jpg 900w, /img/amp-800.jpg 800w,/img/amp-700.jpg 700w, /img/amp-600.jpg 600w, /img/amp-500.jpg 500w, /img/amp-400.jpg 400w,/img/amp-300.jpg 300w, /img/amp-200.jpg 200w, /img/amp-100.jpg 100w">
			</noscript>
		</amp-img>
		<figcaption>Here is a responsive image.</figcaption>
	</figure>
	*/
	//noinspection SpellCheckingInspection
	fn ampImgNode(imageUrlData: &UrlData, imageAbstract: &ImageAbstract, imageMetaData: &ImageMetaData, imageResource: &Resource, pluginData: &MarkdownPluginData, imgNode: UnattachedNode, fallbackText: &str) -> Result<UnattachedNode, CordialError>
	{
		Ok
		(
			"amp-img"
				.with_attributes(Self::imgAttributes(imageUrlData, imageAbstract, imageMetaData, imageResource, pluginData, true)?)
				.with_attribute(AmpLayout::responsive.toAttribute())
				.with_attribute("attribution".str_attribute(&imageMetaData.credit))
				.with_child_element
				(
					"noscript"
						.with_child_element(imgNode)
				)
				.with_child_element
				(
					"div"
						.with_empty_attribute("fallback")
						.with_child_text(fallbackText)
				)
		)
	}
	
	fn imgNode(imageUrlData: &UrlData, imageAbstract: &ImageAbstract, imageMetaData: &ImageMetaData, imageResource: &Resource, pluginData: &MarkdownPluginData) -> Result<UnattachedNode, CordialError>
	{
		let attributes = Self::imgAttributes(imageUrlData, imageAbstract, imageMetaData, imageResource, pluginData, false)?;
		Ok("img".with_attributes(attributes))
	}
	
	fn imgAttributes(imageUrlData: &UrlData, imageAbstract: &ImageAbstract, imageMetaData: &ImageMetaData, imageResource: &Resource, pluginData: &MarkdownPluginData, isForAmp: bool) -> Result<Vec<Attribute>, CordialError>
	{
		let mut attributes = vec!
		[
			"src".str_attribute(imageUrlData.urlOrDataUri.as_str()),
		];
		imageAbstract.addToImgAttributes(&mut attributes);
		pluginData.addImageMetaDataToImgAttributes(&mut attributes, imageMetaData, isForAmp)?;
		imageResource.addToImgAttributes(&mut attributes);
		
		Ok(attributes)
	}
	
	//noinspection SpellCheckingInspection
	fn figcaptionNode(imageAbstract: &ImageAbstract, imageMetaData: &ImageMetaData, pluginData: &MarkdownPluginData) -> Result<UnattachedNode, CordialError>
	{
		/*
			<figcaption>
				<span class="caption">.....</span>
				<small class="credit"><a href="/link/to/licence-url" title="License">Joe Bloggs</small>
			</figcaption>
			
			If the text "Credit:"  is wanted before Joe Bloggs, it can be done with CSS:-
			
			figcaption > small.credit a::before
			{
				content: "Credit:";
			}
			
			// Additionally, for styling
			
			html[dir=ltr] figcaption > .caption
			{
				float: left;
			}
			
			html[dir=ltr] figcaption > .credit
			{
				float: right;
			}
		*/
		
		let (licenseUrl, licenseDescription) = pluginData.imageLicenseUrlAndDescription(imageMetaData)?;
		
		let captionNode = "span"
			.with_class("caption")
			.with_child_text(imageAbstract.caption);
		
		let anchorNode = "a"
			.with_class("caption")
			.with_href_attribute(licenseUrl.as_str())
			.with_title_attribute(licenseDescription)
			.with_child_text(imageMetaData.credit.as_str())
			.with_attribute("target".str_attribute("_blank"))
			.with_attribute("rel".space_separated_attribute(&["license", "noopener", "noreferrer"]));
		
		let (firstChild, secondChild) = if pluginData.renderRightToLeft()
		{
			(anchorNode, captionNode)
		}
		else
		{
			(captionNode, anchorNode)
		};
		
		let figcaptionNode = "figcaption".with_child_element
		(
			"small"
			.with_class("credit")
			.with_child_element(firstChild)
			.with_child_element(secondChild)
		);
		
		Ok(figcaptionNode)
	}
}
