// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct DataSassContext(*mut Sass_Data_Context, Rc<FunctionList>);

impl Drop for DataSassContext
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.delete()
	}
}

impl DataSassContext
{
	#[inline(always)]
	pub(crate) fn new<'p, P: AsRef<Path>>(data: &str, usefulSassOptions: &UsefulSassOptions<'p, P>) -> Self
	{
		let cStr = CString::new(data).unwrap();
		let dataContext = unsafe { sass_make_data_context(strdup(cStr.as_ptr())) };
		
		let options = dataContext.get_options();
		options.set_output_style(usefulSassOptions.output_style);
		options.set_source_comments(usefulSassOptions.source_comments);
		options.set_precision(usefulSassOptions.precision);
		if usefulSassOptions.input_syntax == InputSyntax::SASS
		{
			options.set_is_indented_syntax_src();
		}
		options.set_include_path(usefulSassOptions.include_paths);
		options.set_c_functions(usefulSassOptions.function_list.0);
		
		DataSassContext(dataContext, usefulSassOptions.function_list.clone())
	}
	
	pub(crate) fn compile(&mut self) -> Result<String, Cow<'static, str>>
	{
		self.0.compile();
		let context = self.get_context();
		
		let error_status = context.get_error_status();
		if error_status == 0
		{
			let output_string = context.get_output_string();
			Ok(Self::to_string(output_string))
		}
		else
		{
			let error_message = context.get_error_message();
			if error_message.is_null()
			{
				Err(Cow::Borrowed("libsass failed to compile a data context but didn't return an error message"))
			}
			else
			{
				Err(Cow::Owned(Self::to_string(error_message)))
			}
		}
	}
	
	#[inline(always)]
	fn get_context(&self) -> *mut Sass_Context
	{
		self.0.get_context()
	}
	
	#[inline(always)]
	fn to_string(c_buf: *const c_char) -> String
	{
		unsafe { CStr::from_ptr(c_buf) }.to_string_lossy().into_owned()
	}
}
