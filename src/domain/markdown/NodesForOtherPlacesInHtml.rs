// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) struct NodesForOtherPlacesInHtml
{
	ampScriptNodesForHead: BTreeMap<&'static str, UnattachedNode>,
	hiddenBodyNodes: OrderMap<String, UnattachedNode>,
}

impl NodesForOtherPlacesInHtml
{
	#[inline(always)]
	pub(crate) fn new() -> Self
	{
		Self
		{
			ampScriptNodesForHead: Default::default(),
			hiddenBodyNodes: Default::default(),
		}
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
	pub(crate) fn headAndHiddenBodyHtml(self) -> (String, String)
	{
		let mut headHtml = String::new();
		for (_, node) in self.ampScriptNodesForHead.iter()
		{
			headHtml.push_str(&node.clone().to_html_fragment());
		}
		
		let mut hiddenBodyHtml = String::new();
		for (_, node) in self.hiddenBodyNodes.iter()
		{
			hiddenBodyHtml.push_str(&node.clone().to_html_fragment());
		}
		
		(headHtml, hiddenBodyHtml)
	}
}
