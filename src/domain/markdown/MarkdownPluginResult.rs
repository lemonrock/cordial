// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct MarkdownPluginResult(Either<Vec<UnattachedNode>, String>);

impl MarkdownPluginResult
{
	#[inline(always)]
	pub(crate) fn fromHtmlFragment(htmlFragment: String) -> Result<MarkdownPluginResult, CordialError>
	{
		Ok(MarkdownPluginResult(Right(htmlFragment)))
	}
	
	#[inline(always)]
	pub(crate) fn ok(nodes: Vec<UnattachedNode>) -> Result<MarkdownPluginResult, CordialError>
	{
		Ok(MarkdownPluginResult(Left(nodes)))
	}
	
	#[inline(always)]
	pub(crate) fn nodesToHtmlBytes(self) -> Vec<u8>
	{
		match self.0
		{
			Left(nodes) => nodes.to_html_fragment().into_bytes(),
			Right(htmlFragment) => htmlFragment.into_bytes(),
		}
	}
}
