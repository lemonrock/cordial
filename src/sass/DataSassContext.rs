// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct DataSassContext(*mut Sass_Data_Context);

impl Drop for DataSassContext
{
	fn drop(&mut self)
	{
		unsafe { sass_delete_data_context(self.0) }
	}
}

impl DataSassContext
{
	#[inline(always)]
	pub(crate) fn new<'p, P: AsRef<Path>>(data: &str, usefulSassOptions: &UsefulSassOptions<'p, P>) -> Self
	{
		let cStr = CString::new(data).unwrap();
		let this = DataSassContext(Sass_Data_ContextExt::make(&cStr));
		this.set_options(usefulSassOptions);
		this
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
	fn set_options<'p, P: AsRef<Path>>(&self, usefulSassOptions: &UsefulSassOptions<'p, P>)
	{
		usefulSassOptions.set(self.get_options());
	}
	
	#[inline(always)]
	fn get_options(&self) -> *mut Sass_Options
	{
		self.0.get_options()
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
