// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) fn hjsonMerge(parent: &HjsonValue, child: &HjsonValue, arrayMergeStrategy: ArrayMergeStrategy) -> HjsonValue
{
	use self::HjsonValue::*;
	match *parent
	{
		Null => child.clone(),
		Bool(_) => child.clone(),
		I64(_) => child.clone(),
		U64(_) => child.clone(),
		F64(_) => child.clone(),
		String(_) => child.clone(),
		Array(ref parentVectorOfValues) =>
		{
			if child.is_array()
			{
				let childVectorOfValues = child.as_array().unwrap();
				HjsonValue::Array(arrayMergeStrategy.merge(parentVectorOfValues, childVectorOfValues))
			}
			else
			{
				child.clone()
			}
		}
		Object(ref parentMapOfStringKeysToValues) =>
		{
			if child.is_object()
			{
				let mut mergedMapOfStringKeysToValues = parentMapOfStringKeysToValues.clone();
				let childMapOfStringKeysToValues = child.as_object().unwrap();
				for (key, childValue) in childMapOfStringKeysToValues.iter()
				{
					match parentMapOfStringKeysToValues.get(key)
					{
						None => mergedMapOfStringKeysToValues.insert(key.to_owned(), childValue.clone()),
						Some(parentValue) => mergedMapOfStringKeysToValues.insert(key.to_owned(), hjsonMerge(parentValue, childValue, arrayMergeStrategy))
					};
				}
				HjsonValue::Object(mergedMapOfStringKeysToValues)
			}
			else
			{
				child.clone()
			}
		}
	}
}
