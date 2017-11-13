// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait InputFormat
{
	#[inline(always)]
	fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		resourceInputContentFileNamesWithExtensionFrom(resourceInputName,self.fileExtensions())
	}
	
	#[inline(always)]
	fn fileExtensions(&self) -> &'static [&'static str];
	
	#[inline(always)]
	fn allFileExtensions() -> &'static [&'static str];
}

impl<I: InputFormat> InputFormat for Option<I>
{
	#[inline(always)]
	fn fileExtensions(&self) -> &'static [&'static str]
	{
		match *self
		{
			Some(ref inputFormat) => inputFormat.fileExtensions(),
			None => Self::allFileExtensions(),
		}
	}
	
	#[inline(always)]
	fn allFileExtensions() -> &'static [&'static str]
	{
		I::allFileExtensions()
	}
}

#[inline(always)]
fn resourceInputContentFileNamesWithExtensionFrom(resourceInputName: &str, fileExtensions: &'static [&'static str]) -> Vec<String>
{
	let mut result = Vec::with_capacity(fileExtensions.len());
	
	let first = resourceInputName.rmatch_indices(".").next();
	
	for fileExtension in fileExtensions.iter()
	{
		let index = first.unwrap().0;
		let mut withExtension = String::with_capacity(index + fileExtension.len());
		let slice = if first.is_some()
		{
			&resourceInputName[0..index]
		}
		else
		{
			resourceInputName
		};
		withExtension.push_str(slice);
		withExtension.push_str(fileExtension);
		
		result.push(withExtension);
	}
	
	result
}
