// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;

#[inline(always)]
pub(crate) fn newLua<'lua>() -> Lua<'lua>
{
	let mut lua = Lua::new();
	lua.open_base();
	lua.open_bit32();
	lua.open_coroutine();
	lua.open_math();
	lua.open_package();
	lua.open_string();
	lua.open_table();
	
	lua
}

fn xx() -> Result<(), CordialError>
{
	let mut lua = self::newLua();
	
	lua.set("err", hlua::function0(move || -> Result<i32, &'static str>
	{
		Err("something wrong happened")
	}));
	
	lua.set("foo", ::hlua::function1(move |a: i32| -> i32
	{
		a * 5
	}));
	
	lua.set("foo2", ::hlua::function2(move |a: i32, b: i32| -> i32
	{
		a * b
	}));
	
	{
		let mut cordialGlobal = lua.empty_array("cordial");
		cordialGlobal.set(1, 10);
		cordialGlobal.set("foo", ::hlua::function1(move |a: i32| -> i32
		{
			a * 5
		}));
		
		// array also has empty_array... so we can easily insert our json data
		
		// some_data is absolutely anything
		
		// Or, if we don't want to copy: hlua::push_userdata(some_data, lua, some_metatable)
		
	}
	
	Ok(())
	
	// Use PathExt.executeFileContentsAsLua("/path/to/lua") to execute Lua code and get a result
}
