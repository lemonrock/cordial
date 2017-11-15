// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum MarkdownPlugin
{
	svgbob
	{
		enable_lens: bool,
	},
	csv,
}

impl MarkdownPlugin
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn register(self, plugins: &mut HashMap<Vec<u8>, MarkdownPlugin>)
	{
		use self::MarkdownPlugin::*;
		
		let name = match self
		{
			svgbob { .. } => "svgbob",
			csv => "csv",
		};
		
		plugins.insert(name.as_bytes().to_vec(), self);
	}
	
	#[inline(always)]
	pub(crate) fn execute(&self, codeBlock: &[u8]) -> Result<Vec<u8>, ()>
	{
		use self::MarkdownPlugin::*;
		
		let string = match *self
		{
			svgbob { enable_lens } => Self::execute_svgbob(codeBlock, enable_lens)?,
			csv => Self::execute_csv(codeBlock)?,
		};
		Ok(string.into_bytes())
	}
	
	//noinspection SpellCheckingInspection
	fn execute_svgbob(codeBlock: &[u8], enable_lens: bool) -> Result<String, ()>
	{
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
		
		let string = match from_utf8(codeBlock)
		{
			Err(_) => return Err(()),
			Ok(string) => string,
		};
		
		let grid = Grid::from_str(string, &Settings::compact());
		
		let svg = grid.get_svg();
		
		let result = if enable_lens
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
		Ok(result)
	}
	
	fn execute_csv(codeBlock: &[u8]) -> Result<String, ()>
	{
		let string = match from_utf8(codeBlock)
		{
			Err(_) => return Err(()),
			Ok(string) => string,
		};
		let mut reader = Reader::from_string(string);
		
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
}
