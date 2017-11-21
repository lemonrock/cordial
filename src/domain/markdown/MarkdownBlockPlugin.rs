// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum MarkdownBlockPlugin
{
	csv,
	
	svgbob,
}

impl MarkdownBlockPlugin
{
	#[inline(always)]
	pub(crate) fn registerAllPlugins() -> HashMap<Vec<u8>, MarkdownBlockPlugin>
	{
		use self::MarkdownBlockPlugin::*;
		
		hashmap!
		{
			b"csv".to_vec() => csv,
			b"svgbob".to_vec() => svgbob,
		}
	}
	
	#[inline(always)]
	pub(crate) fn execute(&self, arguments: &[u8], _markdownPluginData: &MarkdownPluginData, _isForAmp: bool, data: &[u8]) -> Result<Vec<u8>, CordialError>
	{
		use self::MarkdownBlockPlugin::*;
		
		let string = match from_utf8(data)
		{
			Err(_) => return Err(CordialError::Configuration("Markdown block plugins use UTF-8 data".to_owned())),
			Ok(string) => string,
		};
		
		let string = match *self
		{
			csv => Self::csv(arguments, string)?,
			svgbob => Self::svgbob(arguments, string)?,
		};
		Ok(string.into_bytes())
	}
	
	//noinspection SpellCheckingInspection
	fn csv(arguments: &[u8], block: &str) -> Result<String, CordialError>
	{
		if !arguments.is_empty()
		{
			return Err(CordialError::Configuration("Markdown block plugin csv takes no arguments".to_owned()));
		}
		
		let mut reader = Reader::from_string(block);
		
		let mut buffer = String::new();
		buffer.push_str("<table>");
		buffer.push_str("<thead>");
		for headers in reader.byte_headers()
		{
			buffer.push_str("<tr>");
			for header in headers
			{
				buffer.push_str(&format!("<th>{}</th>", String::from_utf8(header).unwrap_or("".into())));
			}
			buffer.push_str("</tr>");
		}
		
		buffer.push_str("</thead>");
		buffer.push_str("</thead>");
		buffer.push_str("<tbody>");
		for records in reader.byte_records().map(|r| r.unwrap())
		{
			buffer.push_str("<tr>");
			for record in records
			{
				buffer.push_str(&format!("<td>{}</td>",String::from_utf8(record).unwrap_or("".into())));
			}
			buffer.push_str("</tr>");
		}
		buffer.push_str("</tbody>");
		buffer.push_str("</table>");
		
		Ok(buffer)
	}
	
	//noinspection SpellCheckingInspection
	fn svgbob(arguments: &[u8], block: &str) -> Result<String, CordialError>
	{
		let enableLens = match arguments
		{
			b"" => false,
			b"enable_lens" => true,
			_ => return Err(CordialError::Configuration("Markdown block plugin svgbob takes no arguments or just 'enable_lens'".to_owned()))
		};
		
		Ok(Self::svgbobFromStr(block, enableLens))
	}
	
	#[inline(always)]
	pub(crate) fn svgbobFromStr(string: &str, enableLens: bool) -> String
	{
		use ::svgbob::Grid;
		use ::svgbob::Settings;
		
		#[inline(always)]
		fn build_cells(text: &Vec<Vec<Option<&String>>>) -> String
		{
			let mut buffer = String::new();
			for lines in text
			{
				for line in lines
				{
					match *line
					{
						Some(ref extantLine) => buffer.push_str(&format!("<div>{}</div>", extantLine)),
						None => buffer.push_str("<div></div>"),
					}
				}
			}
			buffer
		}

		let grid = Grid::from_str(string, &Settings::compact());

		let svg = grid.get_svg();

		let result = if enableLens
		{
			let (width, height) = grid.get_size();
			let text = grid.get_all_text();
			let cells = build_cells(&text);
			let lens = format!("<div class='lens'><div class='content' style='width:{}px;height:{}px'>{}</div></div>", width, height, cells);
			format!("<div class='bob_container' style='width:{}px;height:{}px'>{}{}</div>", width, height, svg, lens)
		}
		else
		{
			format!("{}", svg)
		};
		result
	}
}
