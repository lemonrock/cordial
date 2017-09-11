// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Clone)]
pub(crate) struct Webserver
{
}

#[allow(deprecated)] use ::tokio_core::io::IoFuture;
use ::tokio_core::reactor::Core;
use ::tokio_core::reactor::Handle;
use ::tokio_signal::unix::{Signal, SIGHUP, SIGINT, SIGTERM};

impl Webserver
{
	pub(crate) fn start(updatableTlsServerConfigurationFactory: Arc<UpdatableTlsServerConfigurationFactory>, httpSocket: &ServerSocket, httpsSocket: &ServerSocket, httpRequestHandlerFactory: Arc<UpdatableRequestHandlerFactory<HttpRedirectToHttpsRequestHandler>>, httpsRequestHandlerFactory: Arc<UpdatableRequestHandlerFactory<HttpsStaticRequestHandler>>, settings: Settings) -> io::Result<()>
	{
		let respondsToCtrlC = settings.respondsToCtrlC();
		
		let mut core = Core::new().unwrap();
		let handle = core.handle();
		
		// Reconfiguration
		{
			// Also SIGUSR1 and SIGUSR2
			// We should consider sending ourselves a signal in the event configuration is bad...
			let updatableTlsServerConfigurationFactory = updatableTlsServerConfigurationFactory.clone();
			let httpRequestHandlerFactory = httpRequestHandlerFactory.clone();
			let httpsRequestHandlerFactory = httpsRequestHandlerFactory.clone();
			let reconfigure = Self::flattenedSignalStream(SIGHUP, &handle);
			let future = reconfigure.for_each(move |_signal|
			{
				if let Err(error) = settings.reconfigure(&updatableTlsServerConfigurationFactory, &httpRequestHandlerFactory, &httpsRequestHandlerFactory)
				{
					error!("{}", error);
				}
				
				Ok(())
			}).map_err(|_| ());
			handle.spawn(future);
		}
		
		Self::http(&handle, httpSocket, httpRequestHandlerFactory)?;
		
		Self::https(&handle, httpsSocket, httpsRequestHandlerFactory, updatableTlsServerConfigurationFactory)?;
		
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
	
	fn http<R: 'static + RequestHandlerFactory>(handle: &Handle, httpSocket: &ServerSocket, httpRequestHandlerFactory: Arc<R>) -> io::Result<()>
	{
		let (port, cloneOfHandle) = Self::cloneForOuterClosure(httpSocket, &handle);
		Self::forEachIncomingClient(httpSocket, &handle, move |(tcpStream, clientSocketAddress)|
		{
			let requestHandler = httpRequestHandlerFactory.produce();
			
			Self::handlerHttp(&cloneOfHandle, tcpStream, clientSocketAddress, "http", port, requestHandler)
		})
	}
	
	fn https<R: 'static + RequestHandlerFactory>(handle: &Handle, httpsSocket: &ServerSocket, httpsRequestHandlerFactory: Arc<R>, updatableTlsServerConfigurationFactory: Arc<UpdatableTlsServerConfigurationFactory>) -> io::Result<()>
	{
		let (port, cloneOfHandle) = Self::cloneForOuterClosure(httpsSocket, &handle);
		Self::forEachIncomingClient(httpsSocket, &handle, move |(tcpStream, clientSocketAddress)|
		{
			let requestHandler = httpsRequestHandlerFactory.produce();
			
			let tlsServerConfiguration = updatableTlsServerConfigurationFactory.produce();
			let handle = cloneOfHandle.clone();
			cloneOfHandle.spawn(tlsServerConfiguration.accept_async(tcpStream).and_then(move |tlsStream| Self::handlerHttp(&handle, tlsStream, clientSocketAddress, "https", port, requestHandler)).map_err(|_| ()));
			Ok(())
		})
	}
	
	#[allow(deprecated)]
	fn flattenedSignalStream(signal: i32,  handle: &Handle) -> FlattenStream<IoFuture<Signal>>
	{
		Signal::new(signal, &handle).flatten_stream()
	}
	
	fn cloneForOuterClosure(serverSocket: &ServerSocket, handle: &Handle) -> (u16, Handle)
	{
		(serverSocket.port(), handle.clone())
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
	
	#[inline(always)]
	fn handlerHttp<I: 'static + AsyncRead + AsyncWrite, R: 'static + RequestHandler>(handle: &Handle, stream: I, clientSocketAddress: SocketAddr, scheme: &'static str, port: u16, requestHandler: Arc<R>) -> io::Result<()>
	{
		let mut httpServer = Http::new();
		httpServer.keep_alive(requestHandler.httpKeepAlive());
		httpServer.bind_connection(handle, stream, clientSocketAddress, HttpService::new(scheme, port, requestHandler));
		Ok(())
	}
}
