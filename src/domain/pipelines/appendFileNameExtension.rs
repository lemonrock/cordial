// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn appendFileNameExtension<'a>(withoutFileNameExtension: &str, extension: &str) -> String
{
	let mut string = String::with_capacity(withoutFileNameExtension.len() + extension.len());
	string.push_str(withoutFileNameExtension.as_ref());
	string.push_str(extension);
	string
}
