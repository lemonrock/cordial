// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct SassValue(*mut Sass_Value, bool);

impl Drop for SassValue
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.1 && !self.0.is_null()
		{
			self.0.delete()
		}
	}
}

impl Clone for SassValue
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		SassValue(unsafe { sass_clone_value(self.0 as *const _) }, true)
	}
}

impl SassValue
{
	#[inline(always)]
	pub(crate) fn is_null(&self) -> bool
	{
		unsafe { sass_value_is_null(self.0 as *const _) }
	}
	
	/// Returns an error if this is not a boolean or null
	#[inline(always)]
	pub(crate) fn get_number<'a>(&'a self) -> Result<Option<NumberSassValue<'a>>, ()>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_number(self.0 as *const _) }
		{
			Ok(Some(NumberSassValue { reference: self }))
		}
		else
		{
			Err(())
		}
	}
	
	/// Returns an error if this is not a boolean or null
	#[inline(always)]
	pub(crate) fn get_boolean<'a>(&'a self) -> Result<Option<BooleanSassValue<'a>>, ()>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_boolean(self.0 as *const _) }
		{
			Ok(Some(BooleanSassValue { reference: self }))
		}
		else
		{
			Err(())
		}
	}
	
	/// Returns an error if this is not a warning or null
	#[inline(always)]
	pub(crate) fn get_warning<'a>(&'a self) -> Result<Option<WarningSassValue<'a>>, ()>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_warning(self.0 as *const _) }
		{
			Ok(Some(WarningSassValue { reference: self }))
		}
		else
		{
			Err(())
		}
	}
	
	/// Returns an error if this is not a error or null
	#[inline(always)]
	pub(crate) fn get_error<'a>(&'a self) -> Result<Option<ErrorSassValue<'a>>, ()>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_error(self.0 as *const _) }
		{
			Ok(Some(ErrorSassValue { reference: self }))
		}
		else
		{
			Err(())
		}
	}
	
	/// Returns an error if this is not a string or null
	#[inline(always)]
	pub(crate) fn get_string<'a>(&'a self) -> Result<Option<StringSassValue<'a>>, ()>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_string(self.0 as *const _) }
		{
			Ok(Some(StringSassValue { reference: self }))
		}
		else
		{
			Err(())
		}
	}
	
	/// Returns an error if this is not a color or null
	#[inline(always)]
	pub(crate) fn get_color<'a>(&'a self) -> Result<Option<ColorSassValue<'a>>, ()>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_color(self.0 as *const _) }
		{
			Ok(Some(ColorSassValue { reference: self }))
		}
		else
		{
			Err(())
		}
	}
	
	#[inline(always)]
	pub(crate) fn transfer_ownership_to_c(mut self) -> *mut Sass_Value
	{
		self.1 = false;
		let pointer = self.0;
		unsafe { forget(self) };
		pointer
	}
	
	#[inline(always)]
	pub(crate) fn new_null() -> Self
	{
		SassValue(unsafe { sass_make_null() }, true)
	}
	
	#[inline(always)]
	pub(crate) fn new_boolean(value: bool) -> Self
	{
		SassValue(unsafe { sass_make_boolean(value) }, true)
	}
	
	#[inline(always)]
	pub(crate) fn new_number(value: f64, unit: *const c_char) -> Self
	{
		SassValue(unsafe { sass_make_number(value, unit) }, true)
	}
	
	#[inline(always)]
	pub(crate) fn new_string(value: *const c_char) -> Self
	{
		SassValue(unsafe { sass_make_string(value) }, true)
	}
	
	#[inline(always)]
	pub(crate) fn new_quoted_string(value: *const c_char) -> Self
	{
		SassValue(unsafe { sass_make_qstring(value) }, true)
	}
	
	#[inline(always)]
	pub(crate) fn new_warning(message: *const c_char) -> Self
	{
		SassValue(unsafe { sass_make_warning(message) }, true)
	}
	
	#[inline(always)]
	pub(crate) fn new_error(message: *const c_char) -> Self
	{
		SassValue(unsafe { sass_make_error(message) }, true)
	}
	
	#[inline(always)]
	pub(crate) fn new_color(red: f64, green: f64, blue: f64, alpha: f64) -> Self
	{
		SassValue(unsafe { sass_make_color(red, green, blue, alpha) }, true)
	}
}


/*



bool sass_value_is_map (const union Sass_Value* v);
union Sass_Value* sass_make_map     (size_t len);

// Getter for the number of items in map
size_t sass_map_get_length (const union Sass_Value* v);
// Getters and setters for Sass_Map keys and values
union Sass_Value* sass_map_get_key (const union Sass_Value* v, size_t i);
void sass_map_set_key (union Sass_Value* v, size_t i, union Sass_Value*);
union Sass_Value* sass_map_get_value (const union Sass_Value* v, size_t i);
void sass_map_set_value (union Sass_Value* v, size_t i, union Sass_Value*);



// Execute an operation for two Sass_Values and return the result as a Sass_Value too
union Sass_Value* sass_value_op (enum Sass_OP op, const union Sass_Value* a, const union Sass_Value* b);




bool sass_value_is_list (const union Sass_Value* v);
union Sass_Value* sass_make_list    (size_t len, enum Sass_Separator sep, bool is_bracketed);

// Getters and setters for Sass_List
enum Sass_Separator sass_list_get_separator (const union Sass_Value* v);
void sass_list_set_separator (union Sass_Value* v, enum Sass_Separator value);

// Getters and setters for Sass_List values
union Sass_Value* sass_list_get_value (const union Sass_Value* v, size_t i);
void sass_list_set_value (union Sass_Value* v, size_t i, union Sass_Value* value);




*/
