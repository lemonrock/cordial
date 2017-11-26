// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


trait AstNodeExt<'a>
{
	fn useMarkdownAstNodeRecursively<F: FnMut(&'a Self) -> Result<(), CordialError>>(&'a self, nodeUser: &mut F) -> Result<(), CordialError>;
}

impl<'a> AstNodeExt<'a> for AstNode<'a>
{
	fn useMarkdownAstNodeRecursively<F: FnMut(&'a Self) -> Result<(), CordialError>>(&'a self, nodeUser: &mut F) -> Result<(), CordialError>
	{
		nodeUser(self)?;
		
		for childNode in self.children()
		{
			childNode.useMarkdownAstNodeRecursively(nodeUser)?;
		}
		
		Ok(())
	}
}
