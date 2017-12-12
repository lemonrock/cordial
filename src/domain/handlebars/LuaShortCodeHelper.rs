// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct LuaShortCodeHelper
{
	pub(crate) luaFolderPath: Arc<PathBuf>,
}

impl HelperDef for LuaShortCodeHelper
{
	fn call(&self, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError>
	{
		let shortCodeName = h.name();
		
		let mut lua = match self.newLua()
		{
			Err(error) => return Err(RenderError::with(error)),
			Ok(lua) => lua,
		};
		
		{
			let mut namedArgumentsTable = lua.empty_array("namedArguments");
			for (parameterName, parameterValue) in h.hash().iter()
			{
				namedArgumentsTable.set(parameterName.as_str(), Self::convertParameterValueToLuaValue(parameterValue));
			}
		}
		
		{
			let mut anonymousArgumentsTable = lua.empty_array("anonymousArguments");
			let mut oneBasedLuaTableIndex = 1;
			for anonymousParameter in h.params().iter()
			{
				anonymousArgumentsTable.set(oneBasedLuaTableIndex, Self::convertParameterValueToLuaValue(anonymousParameter));
				oneBasedLuaTableIndex += 1;
			}
		}
		
		use self::AnyLuaValue::*;
		
		let bytes = match lua.execute::<AnyLuaValue>(&format!("shortcode(\"{}\", namedArguments, unpack(anonymousArguments))", shortCodeName))
		{
			Err(error) => return Err(RenderError::with(error)),
			Ok(anyLuaValue) => match anyLuaValue
			{
				LuaOther => return Err(RenderError::new("LuaOther values are not supported")),
				
				LuaArray(_) => return Err(RenderError::new("LuaArray values are not supported")),
				
				LuaString(string) => string.into_bytes(),
				
				LuaAnyString(bytes) => bytes.0,
				
				LuaNumber(number) => format!("{}", number).into_bytes(),
				
				LuaBoolean(boolean) => if boolean
				{
					b"yes".to_vec()
				}
				else
				{
					b"no".to_vec()
				},
				
				LuaNil => vec![],
			}
		};
		
		rc.writer.write(&bytes)?;
		
		Ok(())
	}
}

impl LuaShortCodeHelper
{
	const shortcodes: &'static str = "shortcodes";
	
	pub(crate) fn registerForAllShortCodes(luaFolderPath: &Arc<PathBuf>, handlebars: &mut Handlebars) -> Result<(), CordialError>
	{
		let shortcodesFolderPath = luaFolderPath.join(Self::shortcodes);
		
		for entry in (&shortcodesFolderPath).read_dir().context(&shortcodesFolderPath)?
		{
			let entry = entry.context(&shortcodesFolderPath)?;
			let path = entry.path();
			
			let metadata = entry.metadata().context(&path)?;
			if metadata.is_file()
			{
				if let Some(osStrExtension) = path.extension()
				{
					if let Some(utf8Extension) = osStrExtension.to_str()
					{
						if utf8Extension == "lua"
						{
							if let Some(fileStem) = path.file_stem()
							{
								if let Some(utf8FileStem) = fileStem.to_str()
								{
									// register
									handlebars.register_helper(utf8FileStem, Box::new
									(
										Self
										{
											luaFolderPath: luaFolderPath.clone(),
										}
									));
								}
							}
						}
					}
				}
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn convertParameterValueToLuaValue(parameterValue: &ContextJson) -> AnyLuaValue
	{
		let jsonValue = parameterValue.value();
		Self::convertJsonValueToLuaValue(jsonValue)
	}
	
	#[inline(always)]
	fn convertJsonValueToLuaValue(jsonValue: &JsonValue) -> AnyLuaValue
	{
		use self::JsonValue::*;
		
		use self::AnyLuaValue::*;
		
		match *jsonValue
		{
			Null => LuaNil,
			Bool(boolean) => LuaBoolean(boolean),
			Number(ref jsonNumber) => LuaNumber(jsonNumber.as_f64().unwrap_or(::std::f64::NAN)),
			String(ref string) => LuaString(string.to_owned()),
			Array(ref jsonValues) =>
			{
				let mut luaValues = Vec::with_capacity(jsonValues.len());
				let mut oneBasedLuaTableIndex = 1;
				for jsonValue in jsonValues.iter()
				{
					luaValues.push((LuaNumber(oneBasedLuaTableIndex as f64), Self::convertJsonValueToLuaValue(jsonValue)));
					oneBasedLuaTableIndex += 1;
				}
				LuaArray(luaValues)
			}
			Object(ref mapOfJsonValues) =>
			{
				let mut luaValues = Vec::with_capacity(mapOfJsonValues.len());
				for (key, jsonValue) in mapOfJsonValues.iter()
				{
					luaValues.push((LuaString(key.to_owned()), Self::convertJsonValueToLuaValue(jsonValue)));
				}
				LuaArray(luaValues)
			}
		}
	}
	
	#[cfg(unix)] const SharedLibraryExtension: &'static str = "so";
	
	#[cfg(windows)] const SharedLibraryExtension: &'static str = "dll";
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn newLua<'lua>(&self) -> Result<Lua<'lua>, CordialError>
	{
		let mut lua = Lua::new();
		lua.open_base();
		lua.open_bit32();
		lua.open_coroutine();
		lua.open_io();
		lua.open_math();
		lua.open_package();
		lua.open_string();
		lua.open_table();
		
		let luaPackagesPath = self.luaPath("packages")?;
		let luaShortcodesPath = self.luaPath(Self::shortcodes)?;
		
		let luaInitialisationCode = format!
		(r#"

package.path = "{luaPackagesPath}/?.lua;{luaPackagesPath}/?/init.lua"

package.cpath = "{luaPackagesPath}/?.{SharedLibraryExtension}"

function shortcode(shortCodeName, namedArguments, ...)
	assert(loadfile("{luaShortcodesPath}/" .. shortCodeName .. ".lua"))(namedArguments, ...)
end

"#,  luaPackagesPath = &luaPackagesPath, SharedLibraryExtension = Self::SharedLibraryExtension, luaShortcodesPath = &luaShortcodesPath);
		
		lua.execute::<()>(&luaInitialisationCode)?;
		Ok(lua)
	}
	
	#[inline(always)]
	fn luaPath(&self, name: &str) -> Result<String, CordialError>
	{
		let luaPath = self.luaFolderPath.join(name);
		match luaPath.into_os_string().into_string()
		{
			Err(_) => return Err(CordialError::Configuration("a component of the luaPath is not valid UTF-8".to_owned())),
			Ok(luaPath) => Ok(luaPath),
		}
	}
}
