// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum MarkdownInlinePlugin
{
	svg,
}

impl MarkdownInlinePlugin
{
	#[inline(always)]
	pub(crate) fn registerAllPlugins() -> HashMap<Vec<u8>, MarkdownInlinePlugin>
	{
		use self::MarkdownInlinePlugin::*;
		
		hashmap!
		{
			b"svg".to_vec() => svg,
		}
	}
	
	#[inline(always)]
	pub(crate) fn execute<'a, ArgumentsIterator: Iterator<Item=&'a [u8]>>(&self, mayBeEmptyArguments: ArgumentsIterator) -> Result<Vec<u8>, ()>
	{
		let nonEmptyArguments = mayBeEmptyArguments.filter(|item| !item.is_empty());
		
		use self::MarkdownInlinePlugin::*;
		
		let string = match *self
		{
			svg => Self::svg(nonEmptyArguments)?,
		};
		Ok(string.into_bytes())
	}
	
	//noinspection SpellCheckingInspection
	fn svg<'a, ArgumentsIterator: Iterator<Item=&'a [u8]>>(arguments: ArgumentsIterator) -> Result<String, ()>
	{
		if arguments.count() != 0
		{
			return Err(());
		}
		
		
		
		
		
		
		
		
		
		
		
		
		
		
		
		
		panic!();
	}
}
