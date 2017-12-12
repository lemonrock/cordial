// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct UsefulSassOptions<'p, P: 'p + AsRef<Path>>
{
	pub(crate) output_style: OutputStyle,
	pub(crate) source_comments: bool,
	pub(crate) precision: u8,
	pub(crate) input_syntax: InputSyntax,
	pub(crate) include_paths: &'p [P],
	pub(crate) function_list: Rc<FunctionList>,
}

impl<'p, P: AsRef<Path>> UsefulSassOptions<'p, P>
{
	#[inline(always)]
	pub(crate) fn compile_data(&self, data: &str) -> Result<String, Cow<'static, str>>
	{
		DataSassContext::new(data, self).compile()
	}
}
