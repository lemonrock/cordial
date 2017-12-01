// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) struct NodesForOtherPlacesInHtml
{
	startHeadNodes: Vec<UnattachedNode>,
	ampScriptNodesForHead: BTreeMap<&'static str, UnattachedNode>,
	endHeadNodes: Vec<UnattachedNode>,
	hiddenBodyNodes: OrderMap<String, UnattachedNode>,
}

impl NodesForOtherPlacesInHtml
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn new
	(
		isForAmp: bool,
		addAmpLink: bool,
		ampLinkIsCanonical: bool,
		pjaxIsSupported: bool,
		configuration: &Configuration,
		htmlDocumentData: &HtmlDocumentData,
		resources: &Resources,
	) -> Result<Self, CordialError>
	{
		let mut startHeadNodes = vec!
		[
			"meta".with_charset_attribute("utf-8"),
			"title".with_child_text(htmlDocumentData.htmlTitle()),
			meta_with_name_and_content("viewport", if isForAmp
			{
				"width=device-width,minimum-scale=1,initial-scale=1"
			}
			else
			{
				"width=device-width,minimum-scale=1,initial-scale=1,shrink-to-fit=no"
			}),
		];
		
		htmlDocumentData.addStartHeadNodes(&mut startHeadNodes, resources)?;
		
		if !isForAmp && pjaxIsSupported
		{
			startHeadNodes.push(meta_with_http_equiv_and_content("X-PJAX-Version", &configuration.deploymentVersion));
		}
		
		if isForAmp
		{
			startHeadNodes.push
			(
				"script"
					.with_async_attribute()
					.with_attribute("src".str_attribute("https://cdn.ampproject.org/v0.js"))
			);
			startHeadNodes.push
			(
				"style"
					.with_amp_boilerplate_attribute()
					.with_child_text
					(
						"body{-webkit-animation:-amp-start 8s steps(1,end) 0s 1 normal both;-moz-animation:-amp-start 8s steps(1,end) 0s 1 normal both;-ms-animation:-amp-start 8s steps(1,end) 0s 1 normal both;animation:-amp-start 8s steps(1,end) 0s 1 normal both}@-webkit-keyframes -amp-start{from{visibility:hidden}to{visibility:visible}}@-moz-keyframes -amp-start{from{visibility:hidden}to{visibility:visible}}@-ms-keyframes -amp-start{from{visibility:hidden}to{visibility:visible}}@-o-keyframes -amp-start{from{visibility:hidden}to{visibility:visible}}@keyframes -amp-start{from{visibility:hidden}to{visibility:visible}}"
					)
			);
			startHeadNodes.push
			(
				"noscript"
					.with_child_element
					(
						"style"
							.with_amp_boilerplate_attribute()
							.with_child_text
							(
								"body{-webkit-animation:none;-moz-animation:none;-ms-animation:none;animation:none}"
							)
					)
			);
		}
		
		let endHeadNodes = htmlDocumentData.endHeadNodes(addAmpLink, ampLinkIsCanonical, resources)?;
		
		Ok
		(
			Self
			{
				startHeadNodes,
				endHeadNodes,
				ampScriptNodesForHead: Default::default(),
				hiddenBodyNodes: Default::default(),
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn ampScript(&mut self, name: &'static str, url: &str)
	{
		self.ampScriptNodesForHead.entry(name).or_insert_with(|| "amp-anim".amp_script(url));
	}
	
	#[inline(always)]
	pub(crate) fn hiddenBody<S: Into<String>>(&mut self, name: S, unattachedNode: UnattachedNode)
	{
		self.hiddenBodyNodes.entry(name.into()).or_insert(unattachedNode);
	}
	
	#[inline(always)]
	pub(crate) fn headAndHiddenBodyHtml(mut self) -> (String, String)
	{
		let mut headHtml = String::new();
		for node in self.startHeadNodes.drain(..)
		{
			headHtml.push_str(&node.to_html_fragment());
		}
		for (_, node) in self.ampScriptNodesForHead.iter()
		{
			headHtml.push_str(&node.clone().to_html_fragment());
		}
		for node in self.endHeadNodes.drain(..)
		{
			headHtml.push_str(&node.to_html_fragment());
		}
		
		let mut hiddenBodyHtml = String::new();
		for (_, node) in self.hiddenBodyNodes.iter()
		{
			hiddenBodyHtml.push_str(&node.clone().to_html_fragment());
		}
		
		(headHtml, hiddenBodyHtml)
	}
}
