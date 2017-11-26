// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub struct HandlebarsTemplateRenderer<'b>(&'b mut Handlebars);

impl<'b> HandlebarsTemplateRenderer<'b>
{
	#[inline(always)]
	pub(crate) fn render<T: Serialize>(&self, name: &str, data: &T) -> Result<String, CordialError>
	{
		Ok(self.0.render(name, data)?)
	}
	
	#[inline(always)]
	pub(crate) fn template_render<T: Serialize>(&self, template_string: &str, data: &T) -> Result<String, CordialError>
	{
		Ok(self.0.template_render(template_string, data)?)
	}
}
