// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Default, Debug, Clone)]
pub(crate) struct MarkdownParser
{
	options: ComrakOptions,
	plugins: HashMap<Vec<u8>, MarkdownPlugin>,
}

impl MarkdownParser
{
	#[inline(always)]
	pub(crate) fn defaultishParse(headerIdPrefixWithTrailingDash: &str, markdown: &str) -> Result<Vec<u8>, CordialError>
	{
		Self::new(headerIdPrefixWithTrailingDash, MarkdownPlugin::registerAllPlugins()).parseMarkdown(markdown)
	}
	
	#[inline(always)]
	pub(crate) fn new(headerIdPrefixWithTrailingDash: &str, plugins: HashMap<Vec<u8>, MarkdownPlugin>) -> Self
	{
		Self
		{
			options: ComrakOptions
			{
				hardbreaks: true,
				github_pre_lang: true,
				width: 0,
				ext_strikethrough: true,
				ext_tagfilter: true,
				ext_table: true,
				ext_autolink: true,
				ext_tasklist: true,
				ext_superscript: false,
				ext_header_ids: Some(headerIdPrefixWithTrailingDash.to_string()),
				ext_footnotes: true,
			},
			plugins,
		}
	}
	
	pub(crate) fn parseMarkdown(&self, markdown: &str) -> Result<Vec<u8>, CordialError>
	{
		let arena = Arena::new();
		
		let root = parse_document(&arena, markdown, &self.options);
		
		root.useMarkdownAstNodeRecursively(&|node|
		{
			use self::NodeValue::*;
			
			let ref mut value = node.data.borrow_mut().value;
			let updatedValue = match value
			{
				&mut CodeBlock(ref codeBlock) =>
				{
					// NOTE: Ideally, this should be in a function but it requires the 'impl Trait' feature to be stable, which isn't currently the case (November 2017).
					const Space: u8 = 32;
					let mut split = codeBlock.info.split(|byte| *byte == Space);
					let languageOrMarkdownPluginName = split.next().unwrap();
					let mut mayBeEmptyArgumentsIterator = split;
					
					match self.plugins.get(languageOrMarkdownPluginName)
					{
						Some(markdownPlugin) =>
						{
							match markdownPlugin.execute(mayBeEmptyArgumentsIterator, &codeBlock.literal)
							{
								Ok(literal_html) => HtmlBlock(NodeHtmlBlock
								{
									literal: literal_html,
									block_type: 0
								}),
								
								Err(_) => CodeBlock(codeBlock.clone()),
							}
						},
						
						None => CodeBlock(codeBlock.clone()),
					}
				}
				
				_ => value.to_owned(),
			};
			*value = updatedValue;
		});
		
		let mut html = Vec::new();
		if format_html(root, &self.options, &mut html).is_err()
		{
			Err(CordialError::CouldNotFormatMarkdownToHtml)
		}
		else
		{
			Ok(html)
		}
	}
}
