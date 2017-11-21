// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Default, Debug, Clone)]
pub(crate) struct MarkdownParser
{
	options: ComrakOptions,
	blockPlugins: HashMap<Vec<u8>, MarkdownBlockPlugin>,
	inlinePlugins: HashMap<Vec<u8>, MarkdownInlinePlugin>,
}

impl MarkdownParser
{
	#[inline(always)]
	pub(crate) fn defaultish(headerIdPrefixWithTrailingDash: &str) -> Self
	{
		Self::new(headerIdPrefixWithTrailingDash, MarkdownBlockPlugin::registerAllPlugins(), MarkdownInlinePlugin::registerAllPlugins())
	}
	
	#[inline(always)]
	pub(crate) fn new(headerIdPrefixWithTrailingDash: &str, blockPlugins: HashMap<Vec<u8>, MarkdownBlockPlugin>, inlinePlugins: HashMap<Vec<u8>, MarkdownInlinePlugin>) -> Self
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
			blockPlugins,
			inlinePlugins,
		}
	}
	
	pub(crate) fn parse<'a>(&self, markdown: &str, pluginData: &'a MarkdownPluginData, isForAmp: bool) -> Result<Vec<u8>, CordialError>
	{
		let arena = Arena::new();
		
		let root = parse_document(&arena, markdown, &self.options);
		
		root.useMarkdownAstNodeRecursively(&|node|
		{
			let updatedValue = match node.data.borrow().value
			{
				CodeBlock(ref codeBlock) => self.replaceCodeBlockWithHtmlFromPluginIfRequired(codeBlock, pluginData, isForAmp)?,
				
				_ => None,
			};
			
			if let Some(updatedValue) = updatedValue
			{
				let ref mut value = node.data.borrow_mut().value;
				*value = updatedValue;
			}
			
			Ok(())
		})?;
		
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
	
	#[inline(always)]
	fn replaceCodeBlockWithHtmlFromPluginIfRequired<'a>(&self, codeBlock: &NodeCodeBlock, pluginData: &'a MarkdownPluginData, isForAmp: bool) -> Result<Option<NodeValue>, CordialError>
	{
		if codeBlock.fenced && codeBlock.info.starts_with(Self::SectionSign)
		{
			Self::usePlugin(&codeBlock.info, &self.blockPlugins, |blockPlugin, arguments| blockPlugin.execute(arguments, pluginData, isForAmp, &codeBlock.literal))
		}
		else if codeBlock.literal.starts_with(Self::SectionSign)
		{
			Self::usePlugin(&codeBlock.literal, &self.inlinePlugins, |inlinePlugin, arguments| inlinePlugin.execute(arguments, pluginData, isForAmp))
		}
		else
		{
			Ok(None)
		}
	}
	
	#[inline(always)]
	fn usePlugin<Plugin, PluginUser: FnOnce(&Plugin, &[u8]) -> Result<Vec<u8>, CordialError>>(functionLine: &[u8], plugins: &HashMap<Vec<u8>, Plugin>, pluginUser: PluginUser) -> Result<Option<NodeValue>, CordialError>
	{
		const Space: u8 = 32;
		
		let remainder = &functionLine[Self::SectionSign.len() .. ];
		let length = remainder.len();
		
		let mut index = 0;
		while index < length
		{
			if unsafe { *remainder.get_unchecked(index) } == Space
			{
				break;
			}
			
			index += 1;
		}
		
		let name = &remainder[ .. index ];
		let result = match plugins.get(name)
		{
			None => None,
			Some(plugin) =>
			Some
			(
				HtmlBlock
				(
					NodeHtmlBlock
					{
						literal:
						{
							let arguments = &remainder[ index + 1 .. ];
							pluginUser(plugin, arguments)?
						},
						block_type: 0
					}
				)
			),
		};
		
		Ok(result)
	}
	
	// U+00A7, §, Section Sign
	const SectionSign: &'static [u8] = b"\xC2\xA7";
}
