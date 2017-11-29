// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub(crate) struct WebAppManifestIconPixelDensity(f32);

impl Default for WebAppManifestIconPixelDensity
{
	#[inline(always)]
	fn default() -> Self
	{
		WebAppManifestIconPixelDensity(Self::DefaultValue)
	}
}

impl PartialEq for WebAppManifestIconPixelDensity
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.to_f32().eq(&other.0)
	}
}

impl Eq for WebAppManifestIconPixelDensity
{
}

impl PartialOrd for WebAppManifestIconPixelDensity
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		self.to_f32().partial_cmp(&other.0)
	}
}

impl Ord for WebAppManifestIconPixelDensity
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.partial_cmp(other).unwrap_or(Ordering::Equal)
	}
}

impl Hash for WebAppManifestIconPixelDensity
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.to_bits().hash(state)
	}
}

impl AsRef<f32> for WebAppManifestIconPixelDensity
{
	#[inline(always)]
	fn as_ref(&self) -> &f32
	{
		&self.0
	}
}

impl Into<f32> for WebAppManifestIconPixelDensity
{
	#[inline(always)]
	fn into(self) -> f32
	{
		self.0
	}
}

impl Deref for WebAppManifestIconPixelDensity
{
	type Target = f32;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl WebAppManifestIconPixelDensity
{
	const DefaultValue: f32 = 1.0;
	
	#[inline(always)]
	fn to_f32(&self) -> f32
	{
		self.0
	}
	
	#[inline(always)]
	pub(crate) fn isDefault(&self) -> bool
	{
		self.0 == Self::DefaultValue
	}
}
