// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct FunctionList(Sass_Function_List, Vec<Box<SassFunctionTraitObject>>);

impl Drop for FunctionList
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.delete()
	}
}

impl FunctionList
{
	#[inline(always)]
	pub(crate) fn new(sass_functions: Vec<SassFunctionTraitObject>) -> Self
	{
		let list = Sass_Function_List::make(sass_functions.len());
		let mut dropSassFunctionsWhenFunctionListDrops = Vec::with_capacity(sass_functions.len());
		let mut index = 0;
		for mut sass_function in sass_functions
		{
			let signature = CString::new(sass_function.signature()).unwrap();
			
			let mut cookieHolder: Box<SassFunctionTraitObject> = Box::new(sass_function);
			let cookie = cookieHolder.as_mut() as *mut _ as *mut ::std::os::raw::c_void;
			
			let functionEntry = unsafe { sass_make_function(signature.as_ptr(), Some(Self::call), cookie) };
			
			dropSassFunctionsWhenFunctionListDrops.push(cookieHolder);
			list.set_list_entry(index, functionEntry);
			index += 1;
		}
		FunctionList(list, dropSassFunctionsWhenFunctionListDrops)
	}
	
	extern "C" fn call(s_args: *const Sass_Value, cb: Sass_Function_Entry, comp: *mut Sass_Compiler) -> *mut Sass_Value
	{
		let cookie = cb.get_cookie();
		let raw_this = cookie as *mut SassFunctionTraitObject;
		let this = unsafe { &mut *raw_this };
		this.callback(s_args, comp)
	}
}
