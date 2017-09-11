// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone, Deserialize)]
pub(crate) struct daemon
{
	#[serde(default)] user: Option<UserNewType>,
	#[serde(default)] group: Option<GroupNewType>,
	#[serde(default = "daemon::http_socket_default")] http_socket: ServerSocket,
	#[serde(default = "daemon::https_socket_default")] https_socket: ServerSocket,
	#[serde(default = "daemon::pid_file_default")] pid_file: PathBuf,
}

impl Default for daemon
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			user: None,
			group: None,
			http_socket: Self::http_socket_default(),
			https_socket: Self::https_socket_default(),
			pid_file: Self::pid_file_default(),
		}
	}
}

impl daemon
{
	#[inline(always)]
	pub (crate) fn daemonizeAndBindSockets(&self, cacheFolderPath: &Path, isDaemon: bool) -> Result<(::std::net::TcpListener, ::std::net::TcpListener), CordialError>
	{
		if isDaemon
		{
			let httpSocket = self.http_socket.clone();
			let httpsSocket = self.https_socket.clone();
			let context = cacheFolderPath.to_path_buf();
			
			let mut daemonize = Daemonize::new().chown_pid_file(true).umask(0o7077).working_directory(cacheFolderPath).pid_file(&self.pid_file).privileged_action(move ||
			{
				let httpSocket = httpSocket.stdNetTcpListener().context(context.clone())?;
				let httpsSocket = httpsSocket.stdNetTcpListener().context(context.clone())?;
				Ok((httpSocket, httpsSocket))
			});
			
			if let Some(ref user) = self.user
			{
				daemonize = daemonize.user(user.0.clone());
			}
			
			if let Some(ref group) = self.group
			{
				daemonize = daemonize.group(group.0.clone());
			}
			
			let innerResult = daemonize.start()?;
			
			return innerResult;
		}
		else
		{
			let httpSocket = self.http_socket.stdNetTcpListener().context(cacheFolderPath)?;
			let httpsSocket = self.https_socket.stdNetTcpListener().context(cacheFolderPath)?;
			Ok((httpSocket, httpsSocket))
		}
	}
	
	#[inline(always)]
	fn http_socket_default() -> ServerSocket
	{
		ServerSocket
		{
			socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
			time_to_live: 0,
			only_v6: false,
			reuse_address: false,
			reuse_port: false,
			backlog: 0,
			linger: None,
		}
	}
	
	#[inline(always)]
	fn https_socket_default() -> ServerSocket
	{
		ServerSocket
		{
			socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8443),
			time_to_live: 0,
			only_v6: false,
			reuse_address: false,
			reuse_port: false,
			backlog: 0,
			linger: None,
		}
	}
	
	#[inline(always)]
	fn pid_file_default() -> PathBuf
	{
		PathBuf::from("/var/run/cordial.pid")
	}
}
