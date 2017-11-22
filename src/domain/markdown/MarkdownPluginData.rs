// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Copy, Clone)]
pub(crate) struct MarkdownPluginData<'a>
{
	pub(crate) resources: &'a Resources,
	pub(crate) configuration: &'a Configuration,
	pub(crate) language: &'a LanguageData<'a>,
}

impl<'a> MarkdownPluginData<'a>
{
	#[inline(always)]
	pub(crate) fn renderRightToLeft(&self) -> bool
	{
		self.language.language.assume_right_to_left_script
	}
	
	#[inline(always)]
	pub(crate) fn primaryIso639Dash1Alpha2Language(&self) -> Iso639Dash1Alpha2Language
	{
		self.configuration.primaryIso639Dash1Alpha2Language()
	}
	
	#[inline(always)]
	pub(crate) fn iso639Dash1Alpha2Language(&self) -> Iso639Dash1Alpha2Language
	{
		self.language.iso639Dash1Alpha2Language
	}
	
	#[inline(always)]
	pub(crate) fn requiredTranslation(&self, requiredTranslation: RequiredTranslation) -> Result<&Rc<String>, CordialError>
	{
		self.language.language.requiredTranslation(requiredTranslation)
	}
	
	#[inline(always)]
	pub(crate) fn image(&'a self, imageResourceUrl: ResourceUrl) -> Result<ImageMarkdownPluginData<'a>, CordialError>
	{
		use self::ResourceTag::*;
		
		match self.resources.get(&imageResourceUrl)
		{
			None => Err(CordialError::Configuration(format!("Could not get image for '{:?}'", imageResourceUrl))),
			Some(resourceRefCell) =>
			{
				let imageResource = resourceRefCell.try_borrow()?;
				
				let imageMetaData = imageResource.imageMetaData()?.clone();
				let imageAbstract = imageMetaData.imageAbstract(self.iso639Dash1Alpha2Language())?.clone();
				
				let primaryImageUrlData = imageResource.urlDataMandatory(self.primaryIso639Dash1Alpha2Language(), Some(self.iso639Dash1Alpha2Language()), &primary_image)?.clone();
				
				let animationPlaceholderImageUrlData = imageResource.urlData(self.primaryIso639Dash1Alpha2Language(), Some(self.iso639Dash1Alpha2Language()), &animation_placeholder(0)).cloned();
				
				Ok
				(
					ImageMarkdownPluginData
					{
						markdownPluginData: self,
						imageAbstract,
						imageMetaData,
						primaryImageUrlData,
						animationPlaceholderImageUrlData,
						imageResource,
					}
				)
			}
		}
	}
}
