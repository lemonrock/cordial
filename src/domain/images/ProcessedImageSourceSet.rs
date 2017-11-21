// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Default, Debug, Clone)]
pub(crate) struct ProcessedImageSourceSet(u32, u32, pub(crate) Vec<(Url, u32)>);

impl ProcessedImageSourceSet
{
	#[inline(always)]
	pub(crate) fn addToImgAttributes(this: &RefCell<Self>, attributes: &mut Vec<Attribute>) -> Result<(), CordialError>
	{
		let mut attribute = String::new();
		let mut afterFirst = false;
		for &(ref url, width) in this.try_borrow()?.2.iter()
		{
			if afterFirst
			{
				attribute.push(',');
			}
			else
			{
				afterFirst = true;
			}
			attribute.push_str(url.as_str());
			attribute.push(' ');
			attribute.push_str(&format!("{}w", width));
		}
		
		attributes.push("srcset".string_attribute(attribute));
		
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn processedImageSourceSet_default() -> RefCell<ProcessedImageSourceSet>
	{
		RefCell::default()
	}
}
