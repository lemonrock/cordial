// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ServerSocket
{
	pub(crate) socket: SocketAddr,
	#[serde(default)] pub(crate) time_to_live: u32,
	#[serde(default)] pub(crate) only_v6: bool,
	#[serde(default)] pub(crate) reuse_address: bool,
	#[serde(default)] pub(crate) reuse_port: bool,
	#[serde(default)] pub(crate) backlog: i32,
	#[serde(default)] pub(crate) linger: Option<Duration>,
}

impl ServerSocket
{
	#[inline(always)]
	pub(crate) fn port(&self) -> u16
	{
		self.socket.port()
	}
	
	#[inline(always)]
	pub(crate) fn stdNetTcpListener(&self) -> io::Result<::std::net::TcpListener>
	{
		let builder = match self.socket
		{
			SocketAddr::V4(_) => TcpBuilder::new_v4()?,
			SocketAddr::V6(_) => TcpBuilder::new_v6()?,
		};
		if self.time_to_live != 0
		{
			builder.ttl(self.time_to_live)?;
		}
		if self.only_v6
		{
			builder.only_v6(true)?;
		}
		if self.reuse_address
		{
			builder.reuse_address(true)?;
		}
		if self.reuse_port
		{
			builder.reuse_port(true)?;
		}
		
		let netTcpListener = builder.bind(self.socket)?.listen(self.backlog)?;
		if self.linger.is_some()
		{
			netTcpListener.set_linger(self.linger)?;
		}
		Ok(netTcpListener)
	}
}
