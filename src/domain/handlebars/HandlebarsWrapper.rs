// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) struct HandlebarsWrapper
{
	handlebars: RefCell<Handlebars>,
}

impl HandlebarsWrapper
{
	#[inline(always)]
	pub(crate) fn new(handlebarsTemplatesFolderPath: &Path, configuration: &Configuration) -> Result<HandlebarsWrapper, CordialError>
	{
		let mut handlebars = Handlebars::new();
		
		// Register any default templates here
		
		// Register any helpers here
		LuaShortCodeHelper::registerForAllShortCodes(configuration, &mut handlebars)?;
		
		// Register any decorators here
		
		if handlebarsTemplatesFolderPath.exists() && handlebarsTemplatesFolderPath.is_dir()
		{
			handlebarsTemplatesFolderPath.registerAllHandlebarsTemplates(&handlebarsTemplatesFolderPath, &mut handlebars)?;
		}
		Ok
		(
			Self
			{
				handlebars: RefCell::new(handlebars)
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn renderHtmlUsingNamedTemplate<T: Serialize>(&self, name: &str, data: &T) -> Result<String, CordialError>
	{
		self.renderWithHtmlEscapeFunction(|templateRenderer| templateRenderer.render(name, data))
	}
	
	pub(crate) fn renderWithHtmlEscapeFunction<Callback: FnMut(HandlebarsTemplateRenderer) -> Result<R, CordialError>, R>(&self, mut callback: Callback) -> Result<R, CordialError>
	{
		let mut handlebars = self.handlebars.borrow_mut();
		callback(HandlebarsTemplateRenderer(handlebars.deref_mut()))
	}
	
	pub(crate) fn renderWithEscapeFunction<EscapeFunction: 'static + Fn(&str) -> String + Send + Sync, Callback: FnMut(HandlebarsTemplateRenderer) -> Result<R, CordialError>, R>(&self, escapeFunction: EscapeFunction, mut callback: Callback) -> Result<R, CordialError>
	{
		let mut handlebars = self.handlebars.borrow_mut();
		
		handlebars.register_escape_fn(escapeFunction);
		let result = callback(HandlebarsTemplateRenderer(handlebars.deref_mut()));
		handlebars.unregister_escape_fn();
		result
	}
}
