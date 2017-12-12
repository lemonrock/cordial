// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn minifyingEmitterConfig() -> EmitterConfig
{
	EmitterConfig
	{
		line_separator: Cow::Borrowed(""),
		indent_string: Cow::Borrowed(""),
		perform_indent: false,
		perform_escaping: true,
		write_document_declaration: true,
		normalize_empty_elements: true,
		cdata_to_characters: true,
		keep_element_names_stack: true,
		autopad_comments: false,
	}
}
