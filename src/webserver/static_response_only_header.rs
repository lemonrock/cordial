// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


macro_rules! static_response_only_header
{
	($struct_name: ident, $header_name: expr, $header_value: expr) =>
	{
		#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
		struct $struct_name(&'static str);
		
		impl Header for $struct_name
		{
			#[inline(always)]
			fn header_name() -> &'static str
			{
				$header_name
			}
			
			#[inline(always)]
			fn parse_header(_raw: &Raw) -> HyperResult<Self>
			{
				Err(HyperErrorHeader)
			}
			
			#[inline(always)]
			fn fmt_header(&self, headerFormatter: &mut HeaderFormatter) -> FormatResult
			{
				headerFormatter.fmt_line(&self.0)
			}
		}
		
		impl Default for $struct_name
		{
			#[inline(always)]
			fn default() -> Self
			{
				Self::Default
			}
		}
		
		impl $struct_name
		{
			const Default: Self = $struct_name($header_value);
		}
	}
}
