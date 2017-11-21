// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


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
	pub(crate) fn image(&self, imageResourceReference: &ResourceReference) -> Result<(Ref<'a, Resource>, &'a ImageMetaData, Rc<UrlData>, &'a ImageAbstract), CordialError>
	{
		match imageResourceReference.get(self.resources)
		{
			None => Err(CordialError::Configuration(format!("image inline plugin resource '{:?}' not found", imageResourceReference))),
			Some(imageResource) =>
			{
				let imageResource = imageResource.try_borrow()?;
				
				let imageMetaData = imageResource.imageMetaData()?;
				let imageUrlData = imageResource.urlData(self.primaryIso639Dash1Alpha2Language(), Some(self.iso639Dash1Alpha2Language()), &imageResourceReference.tag).ok_or_else(|| CordialError::Configuration(format!("image resource '{:?}' urlData missing", imageResourceReference)))?;
				let imageAbstract = imageMetaData.abstract_(self.iso639Dash1Alpha2Language())?;
				
				Ok
				(
					(
						imageResource,
						imageMetaData,
						imageUrlData,
						imageAbstract,
					)
				)
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn addImageMetaDataToImgAttributes(&self, attributes: &mut Vec<Attribute>, imageMetaData: &ImageMetaData, isForAmp: bool) -> Result<(), CordialError>
	{
		let primaryIso639Dash1Alpha2Language = self.primaryIso639Dash1Alpha2Language();
		let iso639Dash1Alpha2Language = Some(self.iso639Dash1Alpha2Language());
		imageMetaData.addToImgAttributes(attributes, &self.resources, primaryIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, isForAmp)
	}
	
	#[inline(always)]
	pub(crate) fn imageLicenseUrlAndDescription(&'a self, imageMetaData: &'a ImageMetaData) -> Result<(Rc<Url>, &'a str), CordialError>
	{
		let primaryIso639Dash1Alpha2Language = self.primaryIso639Dash1Alpha2Language();
		let iso639Dash1Alpha2Language = self.iso639Dash1Alpha2Language();
		imageMetaData.licenseUrlAndDescription(&self.resources, primaryIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)
	}
}
