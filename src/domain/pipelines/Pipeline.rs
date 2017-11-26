// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait Pipeline
{
	#[inline(always)]
	fn imageMetaData(&self) -> Result<&Rc<ImageMetaData>, CordialError>
	{
		Err(CordialError::Configuration("This resource is not an image".to_owned()))
	}
	
	#[inline(always)]
	fn addToImgAttributes(&self, _attributes: &mut Vec<Attribute>) -> Result<(), CordialError>
	{
		Ok(())
	}
	
	#[inline(always)]
	fn processingPriority(&self) -> ProcessingPriority;
	
	#[inline(always)]
	fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		vec![resourceInputName.to_owned()]
	}
	
	#[inline(always)]
	fn is<'a>(&self) -> (bool, bool);
	
	#[inline(always)]
	fn anchorTitleAttribute(&self, _fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, _iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<Option<Rc<String>>, CordialError>
	{
		Ok(None)
	}
	
	#[inline(always)]
	fn execute(&self, resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, handlebars: &HandlebarsWrapper, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, configuration: &Configuration, rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<PipelineResource>, CordialError>;
}
