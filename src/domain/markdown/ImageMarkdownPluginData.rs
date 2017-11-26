// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub struct ImageMarkdownPluginData<'a>
{
	markdownPluginData: &'a MarkdownPluginData<'a>,
	imageResource: Ref<'a, Resource>,
	imageMetaData: Rc<ImageMetaData>,
	primaryImageUrlData: Rc<UrlData>,
	animationPlaceholderImageUrlData: Option<Rc<UrlData>>,
	imageAbstract: Rc<ImageAbstract>,
}

impl<'a> ImageMarkdownPluginData<'a>
{
	#[inline(always)]
	fn caption(&self) -> &str
	{
		self.imageAbstract.caption.as_str()
	}
	
	#[inline(always)]
	fn credit(&self) -> &str
	{
		self.imageMetaData.credit.as_str()
	}
	
	#[inline(always)]
	fn resources(&self) -> &'a Resources
	{
		self.markdownPluginData.resources
	}
	
	#[inline(always)]
	fn fallbackIso639Dash1Alpha2Language(&self) -> Iso639Dash1Alpha2Language
	{
		self.markdownPluginData.fallbackIso639Dash1Alpha2Language()
	}
	
	#[inline(always)]
	fn iso639Dash1Alpha2Language(&self) -> Iso639Dash1Alpha2Language
	{
		self.markdownPluginData.iso639Dash1Alpha2Language()
	}
	
	#[inline(always)]
	fn licenseUrlAndAnchorTitleAttribute(&'a self) -> Result<(Rc<Url>, Rc<String>), CordialError>
	{
		let fallbackIso639Dash1Alpha2Language = self.fallbackIso639Dash1Alpha2Language();
		let iso639Dash1Alpha2Language = self.iso639Dash1Alpha2Language();
		self.imageMetaData.licenseUrlAndAnchorTitleAttribute(self.resources(), fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)
	}
	
	#[inline(always)]
	fn url(&self, isForAnimationPlaceholder: bool) -> Result<&str, CordialError>
	{
		if isForAnimationPlaceholder
		{
			match self.animationPlaceholderImageUrlData
			{
				None => Err(CordialError::Configuration("This is not an animated image".to_owned())),
				Some(ref animationPlaceholderImageUrlData) => Ok(animationPlaceholderImageUrlData.url_str()),
			}
		}
		else
		{
			Ok(self.primaryImageUrlData.url_str())
		}
	}
	
	pub(crate) fn isAnimated(&self) -> bool
	{
		self.animationPlaceholderImageUrlData.is_some()
	}
	
	pub(crate) fn ampAnimNode(&self) -> Result<UnattachedNode, CordialError>
	{
		/*
			<amp-anim width="245" height="300" src="/img/gopher.gif" alt="an animation" attribution="The Go gopher was designed by Reneee French and is licensed under CC 3.0 attributions.">
				<amp-img placeholder width="245" height="300" src="/img/gopher.png"></amp-img>
			</amp-anim>
		*/
		let node = "amp-img"
			.with_attributes(self.imgLikeAttributes(true, false)?)
			.with_attribute(AmpLayout::responsive.toAttribute())
			.with_attribute("attribution".str_attribute(self.credit()))
			.with_child_element(self.ampImgNode(true)?);
		Ok(node)
	}
	
	//noinspection SpellCheckingInspection
	pub(crate) fn ampImgNode(&self, isForAnimationPlaceholder: bool) -> Result<UnattachedNode, CordialError>
	{
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
		
		let node = "amp-img"
			.with_attributes(self.imgLikeAttributes(true, isForAnimationPlaceholder)?)
			.with_attribute(AmpLayout::responsive.toAttribute())
			.with_attribute("attribution".str_attribute(self.credit()))
			.with_child_element
			(
				"noscript"
					.with_child_element(self.imgNode()?)
			)
			.with_child_element
			(
				"div"
					.with_empty_attribute("fallback")
					.with_child_text(self.markdownPluginData.requiredTranslation(RequiredTranslation::missing_image_fallback)?.as_str())
			);
		
		if isForAnimationPlaceholder
		{
			Ok(node.with_attribute("placeholder".empty_attribute()))
		}
		else
		{
			Ok(node)
		}
	}
	
	#[inline(always)]
	pub(crate) fn imgNode(&self) -> Result<UnattachedNode, CordialError>
	{
		Ok("img".with_attributes(self.imgLikeAttributes(false, false)?))
	}
	
	//noinspection SpellCheckingInspection
	pub(crate) fn figcaptionNode(&self) -> Result<UnattachedNode, CordialError>
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
		
		let (licenseUrl, licenseAnchorTitleAttribute) = self.licenseUrlAndAnchorTitleAttribute()?;
		
		let captionNode = "span"
			.with_class("caption")
			.with_child_text(self.caption());
		
		let anchorNode = "a"
			.with_class("caption")
			.with_href_attribute(licenseUrl.as_str())
			.with_title_attribute(licenseAnchorTitleAttribute.as_str())
			.with_child_text(self.credit())
			.with_attribute("target".str_attribute("_blank"))
			.with_attribute("rel".space_separated_attribute(&["license", "noopener", "noreferrer"]));
		
		let (firstChild, secondChild) = if self.markdownPluginData.renderRightToLeft()
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
	
	#[inline(always)]
	fn imgLikeAttributes(&self, isForAmp: bool, isForAnimationPlaceholder: bool) -> Result<Vec<Attribute>, CordialError>
	{
		let mut attributes = vec!
		[
			"src".str_attribute(self.url(isForAnimationPlaceholder)?),
		];
		self.imageAbstract.addToImgAttributes(&mut attributes);
		self.addImageMetaDataToImgAttributes(&mut attributes, isForAmp)?;
		self.imageResource.addToImgAttributes(&mut attributes)?;
		
		Ok(attributes)
	}
	
	#[inline(always)]
	fn addImageMetaDataToImgAttributes(&self, attributes: &mut Vec<Attribute>, isForAmp: bool) -> Result<(), CordialError>
	{
		let fallbackIso639Dash1Alpha2Language = self.fallbackIso639Dash1Alpha2Language();
		let iso639Dash1Alpha2Language = Some(self.iso639Dash1Alpha2Language());
		self.imageMetaData.addToImgAttributes(attributes, self.resources(), fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, isForAmp)
	}
}
