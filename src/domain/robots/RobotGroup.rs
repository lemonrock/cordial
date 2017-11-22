// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RobotGroup
{
	#[serde(default = "RobotGroup::user_agents_default")] user_agents: HashSet<String>,
	#[serde(default)] crawl_delay: u64,
	#[serde(default = "RobotGroup::directives_default")] directives: Vec<RobotDirective>,
}

impl Default for RobotGroup
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			user_agents: Self::user_agents_default(),
			crawl_delay: 0,
			directives: Self::directives_default(),
		}
	}
}

impl RobotGroup
{
	#[inline(always)]
	pub(crate) fn writeTo<W: Write>(&self, writer: &mut W, relativeUrlPathForRobotDirective: &Cow<'static, str>) -> io::Result<()>
	{
		for userAgent in self.user_agents.iter()
		{
			writer.write_all(b"User-Agent: ")?;
			writer.write_all(userAgent.as_bytes())?;
			writer.write_all(b"\n")?;
		}
		if self.crawl_delay != 0
		{
			writer.write_all(b"Crawl-Delay: ")?;
			writer.write_all(format!("{}", self.crawl_delay).as_bytes())?;
			writer.write_all(b"\n")?;
		}
		for directive in self.directives.iter()
		{
			writer.write_all(directive.withBaseUrl(relativeUrlPathForRobotDirective).as_bytes())?;
			writer.write_all(b"\n")?;
		}
		writer.write_all(b"\n")?;
		Ok(())
	}
	
	#[inline(always)]
	fn user_agents_default() -> HashSet<String>
	{
		hashset!
		{
			"*".to_owned(),
		}
	}
	
	#[inline(always)]
	fn directives_default() -> Vec<RobotDirective>
	{
		vec![RobotDirective::Allow("*".to_owned()), RobotDirective::Disallow("/*?".to_owned())]
	}
}
