// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Clone)]
pub struct Webserver
{
}

#[allow(deprecated)] use ::tokio_core::io::IoFuture;
use ::tokio_core::reactor::Core;
use ::tokio_core::reactor::Handle;
use ::tokio_signal::unix::{Signal, SIGHUP, SIGINT, SIGTERM};

impl Webserver
{
	pub fn start<HttpR: 'static + RequestHandler, HttpsR: 'static + RequestHandler>(tlsServerConfiguration: Arc<ServerConfig>, httpSocket: &ServerSocket, httpsSocket: &ServerSocket, httpKeepAlive: bool, ourHostNames: Arc<HashSet<String>>, httpRequestHandler: Arc<HttpR>, httpsRequestHandler: Arc<HttpsR>, respondsToCtrlC: bool) -> io::Result<()>
	{
		let mut core = Core::new().unwrap();
		let handle = core.handle();
		
		// Reconfiguration
		{
			// Also SIGUSR1 and SIGUSR2
			// We should consider sending ourselves a signal in the event configuration is bad...
			let reconfigure = Self::flattenedSignalStream(SIGHUP, &handle);
			let future = reconfigure.for_each(|_signal|
			{
				eprintln!("TODO: Reload configuration");
				Ok(())
			}).map_err(|_| ());
			handle.spawn(future);
		}
		
		Self::http(&handle, &ourHostNames, httpKeepAlive, httpSocket, httpRequestHandler)?;
		
		Self::https(&handle, &ourHostNames, httpKeepAlive, httpsSocket, httpsRequestHandler, tlsServerConfiguration)?;
		
		// Run the event loop until terminated
		if respondsToCtrlC
		{
			let signalsThatTerminate = Self::flattenedSignalStream(SIGINT, &handle).select(Self::flattenedSignalStream(SIGTERM, &handle)).into_future();
			let (item, _error) = core.run(signalsThatTerminate).ok().unwrap();
			match item.unwrap()
			{
				SIGINT => (),
				SIGTERM => (),
				_ => unreachable!(),
			}
		}
		else
		{
			let signalsThatTerminate = Self::flattenedSignalStream(SIGINT, &handle).into_future();
			let (item, _error) = core.run(signalsThatTerminate).ok().unwrap();
			match item.unwrap()
			{
				SIGTERM => (),
				_ => unreachable!(),
			}
		}
		
		Ok(())
	}
	
	fn http<R: 'static + RequestHandler>(handle: &Handle, ourHostNames: &Arc<HashSet<String>>, httpKeepAlive: bool, httpSocket: &ServerSocket, httpRequestHandler: Arc<R>) -> io::Result<()>
	{
		let (port, cloneOfHandle, ourHostNames) = Self::cloneForOuterClosure(httpSocket, &handle, &ourHostNames);
		Self::forEachIncomingClient(httpSocket, &handle, move |(tcpStream, clientSocketAddress)|
		{
			let (ourHostNames, requestHandler) = Self::cloneForInnerClosure(&ourHostNames, &httpRequestHandler);
			
			Self::handlerHttp(httpKeepAlive, &cloneOfHandle, tcpStream, clientSocketAddress, "http", port, ourHostNames, requestHandler)
		})
	}
	
	fn https<R: 'static + RequestHandler>(handle: &Handle, ourHostNames: &Arc<HashSet<String>>, httpKeepAlive: bool, httpsSocket: &ServerSocket, httpsRequestHandler: Arc<R>, tlsServerConfiguration: Arc<ServerConfig>) -> io::Result<()>
	{
		let (port, cloneOfHandle, ourHostNames) = Self::cloneForOuterClosure(httpsSocket, &handle, &ourHostNames);
		Self::forEachIncomingClient(httpsSocket, &handle, move |(tcpStream, clientSocketAddress)|
		{
			let (ourHostNames, requestHandler) = Self::cloneForInnerClosure(&ourHostNames, &httpsRequestHandler);
			
			let handle = cloneOfHandle.clone();
			cloneOfHandle.spawn(tlsServerConfiguration.accept_async(tcpStream).and_then(move |tlsStream| Self::handlerHttp(httpKeepAlive, &handle, tlsStream, clientSocketAddress, "https", port, ourHostNames, requestHandler)).map_err(|_| ()));
			Ok(())
		})
	}
	
	#[allow(deprecated)]
	fn flattenedSignalStream(signal: i32,  handle: &Handle) -> FlattenStream<IoFuture<Signal>>
	{
		Signal::new(signal, &handle).flatten_stream()
	}
	
	fn cloneForOuterClosure(serverSocket: &ServerSocket, handle: &Handle, ourHostNames: &Arc<HashSet<String>>) -> (u16, Handle, Arc<HashSet<String>>)
	{
		(serverSocket.port(), handle.clone(), ourHostNames.clone())
	}
	
	#[allow(deprecated)]
	fn forEachIncomingClient<F, U>(serverSocket: &ServerSocket, handle: &Handle, closure: F) -> io::Result<()>
	where F: 'static + FnMut((::tokio_core::net::TcpStream, SocketAddr)) -> U,
		  U: 'static + IntoFuture<Item=(), Error=io::Error>,
	{
		let serverFuture = serverSocket.tokioCoreNetTcpListener(handle)?.incoming().for_each(closure).map_err(|_| ());
		handle.spawn(serverFuture);
		Ok(())
	}
	
	fn cloneForInnerClosure<R: 'static + RequestHandler>(ourHostNames: &Arc<HashSet<String>>, requestHandler: &Arc<R>) -> (Arc<HashSet<String>>, Arc<R>)
	{
		(ourHostNames.clone(), requestHandler.clone())
	}
	
	#[inline(always)]
	fn handlerHttp<I: 'static + AsyncRead + AsyncWrite, R: 'static + RequestHandler>(httpKeepAlive: bool, handle: &Handle, stream: I, clientSocketAddress: SocketAddr, scheme: &'static str, port: u16, ourHostNames: Arc<HashSet<String>>, requestHandler: Arc<R>) -> io::Result<()>
	{
		let mut httpServer = Http::new();
		httpServer.keep_alive(httpKeepAlive);
		httpServer.bind_connection(handle, stream, clientSocketAddress, HttpService::new(scheme, port, ourHostNames, requestHandler));
		Ok(())
	}
}
