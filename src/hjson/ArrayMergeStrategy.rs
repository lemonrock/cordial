// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ArrayMergeStrategy
{
	/// This strategy mirrors that used for merging objects, as it treats indices as this they were keys
	ByIndex,
	
	/// This strategy appends the children elements after the parent elements
	#[allow(unused)] UnionParentFirst,
	
	/// This strategy appends the children elements before the parent elements
	#[allow(unused)] UnionParentLast,
}

impl ArrayMergeStrategy
{
	pub fn merge(&self, parentVectorOfValues: &Vec<HjsonValue>, childVectorOfValues: &Vec<HjsonValue>) -> Vec<HjsonValue>
	{
		use self::ArrayMergeStrategy::*;
		match *self
		{
			ByIndex =>
			{
				let parentLength = parentVectorOfValues.len();
				let childLength = childVectorOfValues.len();
				let mergedLength = max(parentLength, childLength);
				
				let mut mergedVectorOfValues = Vec::with_capacity(mergedLength);
				
				
				let commonLength = min(parentLength, childLength);
				for index in 0 .. commonLength
				{
					let parentValue = parentVectorOfValues.get(index).unwrap();
					let childValue = childVectorOfValues.get(index).unwrap();
					let mergedValue = hjsonMerge(parentValue, childValue, *self);
					mergedVectorOfValues.push(mergedValue);
				}
				
				if parentLength > childLength
				{
					for index in commonLength .. parentLength
					{
						mergedVectorOfValues.push(parentVectorOfValues.get(index).unwrap().clone())
					}
				}
				else if childLength > parentLength
				{
					for index in commonLength .. childLength
					{
						mergedVectorOfValues.push(childVectorOfValues.get(index).unwrap().clone())
					}
				}
				
				mergedVectorOfValues
			}
			UnionParentFirst =>
			{
				let mut mergedVectorOfValues = parentVectorOfValues.clone();
				mergedVectorOfValues.extend_from_slice(childVectorOfValues);
				
				mergedVectorOfValues
			}
			UnionParentLast =>
			{
				let mut mergedVectorOfValues = childVectorOfValues.clone();
				mergedVectorOfValues.extend_from_slice(parentVectorOfValues);
				
				mergedVectorOfValues
			}
		}
	}
}
