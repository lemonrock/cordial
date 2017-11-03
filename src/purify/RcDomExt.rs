// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub trait RcDomExt
{
	fn verify(&self, context: &Path) -> Result<(), CordialError>;
	
	fn recursivelyStripNodesOfCommentsAndProcessingInstructionAndCreateSaneDocType(&self, context: &Path) -> Result<(), CordialError>;
	
	#[doc(hidden)]
	fn _verifyIsDocumentAndNotAFragment(&self, context: &Path) -> Result<(), CordialError>;
	
	#[doc(hidden)]
	fn _verifyHasNoErrors(&self, context: &Path) -> Result<(), CordialError>;
	
	#[doc(hidden)]
	fn _verifyHasNoQuirks(&self, context: &Path) -> Result<(), CordialError>;
	
	#[doc(hidden)]
	fn _verifyRootElement(&self, context: &Path) -> Result<(), CordialError>;
}

impl RcDomExt for RcDom
{
	fn verify(&self, context: &Path) -> Result<(), CordialError>
	{
		self._verifyIsDocumentAndNotAFragment(context)?;
		self._verifyHasNoQuirks(context)?;
		self._verifyHasNoQuirks(context)?;
		self._verifyRootElement(context)
	}
	
	fn recursivelyStripNodesOfCommentsAndProcessingInstructionAndCreateSaneDocType(&self, context: &Path) -> Result<(), CordialError>
	{
		let document = &self.document;
		document.validateChildrenAndRemoveCommentsAndProcessingInstructions(context);
		
		let doctypeNode = Node
		{
			parent: Cell::new(Some(Rc::downgrade(document))),
			children: RefCell::new(Vec::new()),
			data: Doctype
			{
				name: "html".into(),
				public_id: "".into(),
				system_id: "".into(),
			},
		};
		document.children.borrow_mut().insert(0, Rc::new(doctypeNode));
		Ok(())
	}
	
	#[doc(hidden)]
	fn _verifyIsDocumentAndNotAFragment(&self, context: &Path) -> Result<(), CordialError>
	{
		match self.document.data
		{
			Document => Ok(()),
			_ => Err(CordialError::InvalidFile(context.to_path_buf(), "HTML should be a rooted document".to_owned())),
		}
	}
	
	#[doc(hidden)]
	fn _verifyHasNoErrors(&self, context: &Path) -> Result<(), CordialError>
	{
		if self.errors.is_empty()
		{
			Ok(())
		}
		else
		{
			Err(CordialError::InvalidFile(context.to_path_buf(), format!("HTML parsed with errors '{:?}'", self.errors)))
		}
	}
	
	#[doc(hidden)]
	fn _verifyHasNoQuirks(&self, context: &Path) -> Result<(), CordialError>
	{
		use ::html5ever::tree_builder::QuirksMode;
		
		if self.quirks_mode == QuirksMode::NoQuirks
		{
			Ok(())
		}
		else
		{
			Err(CordialError::InvalidFile(context.to_path_buf(), "HTML should not need quirks for parsing in 2017".to_owned()))
		}
	}
	
	#[doc(hidden)]
	fn _verifyRootElement(&self, context: &Path) -> Result<(), CordialError>
	{
		let mut hasDocType = false;
		let mut hasHtmlRoot = false;
		for childOfDocument in self.document.children.borrow().iter()
		{
			match childOfDocument.data
			{
				Text { .. } => return Err(CordialError::InvalidFile(context.to_path_buf(), "Document nodes are not allowed in the root".to_owned())),
				
				Document => return Err(CordialError::InvalidFile(context.to_path_buf(), "Text nodes are not allowed in the root".to_owned())),
				
				Doctype { ref name, ref public_id, ref system_id } =>
					{
						if hasDocType
						{
							return Err(CordialError::InvalidFile(context.to_path_buf(), "multiple DOCTYPE".to_owned()));
						}
						hasDocType = true;
						if hasHtmlRoot
						{
							return Err(CordialError::InvalidFile(context.to_path_buf(), "DOCTYPE after html root".to_owned()));
						}
						if !name.eq_ignore_ascii_case("html")
						{
							return Err(CordialError::InvalidFile(context.to_path_buf(), format!("Non html DOCTYPE '{}' found in document root", name)));
						}
						if !public_id.is_empty()
						{
							return Err(CordialError::InvalidFile(context.to_path_buf(), format!("Non empty DOCTYPE public id '{}' found in document root", public_id)));
						}
						if !system_id.is_empty()
						{
							return Err(CordialError::InvalidFile(context.to_path_buf(), format!("Non empty DOCTYPE system id '{}' found in document root", system_id)));
						}
					},
				
				Element { ref name, .. } =>
					{
						if !name.local.eq_str_ignore_ascii_case("html")
						{
							return Err(CordialError::InvalidFile(context.to_path_buf(), format!("Non html-element '{:?}' found in document root", name)));
						}
						if hasHtmlRoot
						{
							return Err(CordialError::InvalidFile(context.to_path_buf(), "Multiple html elements in document root".to_owned()));
						}
						hasHtmlRoot = true;
					}
				
				ProcessingInstruction { .. } | Comment { .. } => (), //ignore
			}
		}
		
		Ok(())
	}
}
