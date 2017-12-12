// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait Sass_OptionsExt
{
	#[inline(always)]
	fn set_output_style(self, style: OutputStyle);
	
	#[inline(always)]
	fn set_source_comments(self, on: bool);
	
	#[inline(always)]
	fn set_precision(self, precision: u8);
	
	#[inline(always)]
	fn set_is_indented_syntax_src(self);
	
	#[inline(always)]
	fn set_include_path<P: AsRef<Path>>(self, paths: &[P]);
	
	#[inline(always)]
	fn set_c_functions(self, functions: Sass_Function_List);
}

impl Sass_OptionsExt for *mut Sass_Options
{
	#[inline(always)]
	fn set_output_style(self, output_style: OutputStyle)
	{
		use self::OutputStyle::*;
		use self::Sass_Output_Style::*;
		
		let style = match output_style
		{
			Nested => SASS_STYLE_NESTED,
			Expanded => SASS_STYLE_EXPANDED,
			Compact => SASS_STYLE_COMPACT,
			Compressed => SASS_STYLE_COMPRESSED,
		};
		
		unsafe { sass_option_set_output_style(self, style) }
	}
	
	#[inline(always)]
	fn set_source_comments(self, on: bool)
	{
		unsafe { sass_option_set_source_comments(self, on) }
	}
	
	#[inline(always)]
	fn set_precision(self, precision: u8)
	{
		unsafe { sass_option_set_precision(self, precision as i32) }
	}
	
	#[inline(always)]
	fn set_is_indented_syntax_src(self)
	{
		unsafe { sass_option_set_is_indented_syntax_src(self, true) }
	}
	
	#[inline(always)]
	fn set_include_path<P: AsRef<Path>>(self, paths: &[P])
	{
		if paths.is_empty()
		{
			return;
		}
		
		#[cfg(windows)] const Separator: &'static str = ";";
		#[cfg(not(windows))] const Separator: &'static str = ",";
		
		let mut afterFirst = false;
		let mut joinedPaths = OsString::new();
		for path in paths.iter()
		{
			if afterFirst
			{
				joinedPaths.push(Separator);
			}
			else
			{
				afterFirst = true;
			}
			
			joinedPaths.push(path.as_ref().as_os_str());
		}
		
		if cfg!(windows)
		{
			let string = joinedPaths.into_string().unwrap();
			let cString = CString::new(string).unwrap();
			
			unsafe { sass_option_set_include_path(self, cString.as_ptr()) };
		}
		else
		{
			use ::std::os::unix::ffi::OsStrExt;
			let bytes = joinedPaths.as_bytes();
			
			unsafe { sass_option_set_include_path(self, bytes.as_ptr() as *const _) }
		}
	}
	
	#[inline(always)]
	fn set_c_functions(self, function_list: Sass_Function_List)
	{
		unsafe { sass_option_set_c_functions(self, function_list) }
	}
}
