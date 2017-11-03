// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub trait NodeExt
{
	fn validateChildrenAndRemoveCommentsAndProcessingInstructions(&self, context: &Path) -> Result<(), CordialError>;
}

impl NodeExt for Rc<Node>
{
	fn validateChildrenAndRemoveCommentsAndProcessingInstructions(&self, context: &Path) -> Result<(), CordialError>
	{
		let mut children = self.children.borrow_mut();
		let mut processedChildren = Vec::with_capacity(children.len());
		
		let mut previousWasTextNode = false;
		let mut lastAddedNodeWasTextNode = false;
		for childNode in children.iter()
		{
			match childNode.data
			{
				Comment { .. } | ProcessingInstruction { .. } =>
					{
						previousWasTextNode = false;
					},
				
				Text { ref contents } =>
					{
						if !childNode.children.borrow().is_empty()
						{
							return Err(CordialError::InvalidFile(context.to_path_buf(), "Text nodes must not have children".to_owned()));
						}
						
						if previousWasTextNode
						{
							return Err(CordialError::InvalidFile(context.to_path_buf(), "Text nodes can not have a previous sibling which is also a text node".to_owned()));
						}
						
						// Merge with a previous text node; this occurs because we remove comments and processing instructions
						if lastAddedNodeWasTextNode
						{
							let previousTextNode: Rc<Node> = processedChildren.pop().unwrap();
							match previousTextNode.data
							{
								Text { contents: ref previousNodeContent } =>
									{
										let mergedNode = Node
										{
											parent: Cell::new(Some(Rc::downgrade(self))),
											children: RefCell::new(Vec::new()),
											data: Text
											{
												contents:
												{
													let previousContents = previousNodeContent.borrow();
													let contents = contents.borrow();
													let mut mergedContents: Tendril<UTF8, NonAtomic> = Tendril::with_capacity(previousContents.len32() + contents.len32());
													mergedContents.push_tendril(&previousContents);
													mergedContents.push_tendril(&contents);
													RefCell::new(mergedContents)
												}
											}
										};
										processedChildren.push(Rc::new(mergedNode));
									}
								_ => unreachable!("Previously added a text node"),
							}
						}
						else
						{
							processedChildren.push(childNode.clone());
						}
						previousWasTextNode = true;
						lastAddedNodeWasTextNode = true;
					}
				
				Document | Doctype { .. } =>
					{
						return Err(CordialError::InvalidFile(context.to_path_buf(), "Document and DOCTYPE nodes are not valid children".to_owned()));
					}
				
				Element { ref name, .. } =>
					{
						if name.prefix.is_some()
						{
							return Err(CordialError::InvalidFile(context.to_path_buf(), "HTML 5 elements should not have namespace prefixes".to_owned()));
						}
						
						if !name.ns.is_empty()
						{
							return Err(CordialError::InvalidFile(context.to_path_buf(), "HTML 5 elements should not have namespaces".to_owned()));
						}
						
						childNode.validateChildrenAndRemoveCommentsAndProcessingInstructions(context);
						processedChildren.push(childNode.clone());
						previousWasTextNode = false;
						lastAddedNodeWasTextNode = false;
					}
			}
		}
		
		*children = processedChildren;
		
		Ok(())
	}
}
