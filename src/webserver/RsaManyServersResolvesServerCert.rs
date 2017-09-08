// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) struct RsaManyServersResolvesServerCert
{
	certifiedKeys: OrderMap<String, CertifiedKey>,
}

impl ResolvesServerCert for RsaManyServersResolvesServerCert
{
	//noinspection SpellCheckingInspection
	fn resolve(&self, server_name: Option<&str>, _sigschemes: &[SignatureScheme]) -> Option<CertifiedKey>
	{
		if let Some(serverName) = server_name
		{
			self.certifiedKeys.get(serverName).map(|refCertifiedKey| refCertifiedKey.clone())
		}
		else
		{
			Some(self.certifiedKeys.get_index(0).unwrap().1.clone())
		}
	}
}

impl RsaManyServersResolvesServerCert
{
	pub(crate) fn new(inputFolderPath: &Path, environment: &str, serverHostNamesWithPrimaryFirst: OrderMap<String, ()>) -> Result<Arc<Self>, CordialError>
	{
		let environmentFolderPath = inputFolderPath.join(environment);
		
		let mut certifiedKeys = OrderMap::with_capacity(serverHostNamesWithPrimaryFirst.len());
		for serverName in serverHostNamesWithPrimaryFirst.keys()
		{
			let certifiedKey = Self::loadRsaCertifiedKey(&environmentFolderPath, serverName)?;
			
			certifiedKeys.insert(serverName.to_owned(), certifiedKey);
		}
		
		Ok
		(
			Arc::new
			(
				Self
				{
					certifiedKeys
				}
			)
		)
	}
	
	//noinspection SpellCheckingInspection
	fn loadRsaCertifiedKey(environmentFolderPath: &Path, serverHostName: &str) -> Result<CertifiedKey, CordialError>
	{
		let certificateChain = environmentFolderPath.join(format!("{}.certificates.pem", serverHostName)).fileContentsAsPemX509Certificates()?;
		
		let privateKey = environmentFolderPath.join(format!("{}.private-key.pem", serverHostName)).fileContentsAsPemRsaPrivateKey()?;
		
		let stapledOcspResponse = environmentFolderPath.join(format!("{}.staple.ocsp", serverHostName)).fileContentsAsBytesIfExtant().context(environmentFolderPath)?;
		
		// An optional collection of SCTs from CT logs, proving the certificate is included on those logs. This must be a `SignedCertificateTimestampList` encoding; see RFC6962.
		let scts = environmentFolderPath.join(format!("{}.staple.scts", serverHostName)).fileContentsAsBytesIfExtant().context(environmentFolderPath)?;
		
		// Error is '()'
		let key = match RSASigningKey::new(&privateKey)
		{
			Err(_) => return Err(CordialError::Configuration("RSA signing key is not valid in some way".to_owned())),
			Ok(key) => key,
		};
		let key: Arc<Box<SigningKey>> = Arc::new(Box::new(key));
		
		let mut certifiedKey = CertifiedKey::new(certificateChain, key);
		certifiedKey.ocsp = stapledOcspResponse;
		certifiedKey.sct_list = scts;
		Ok(certifiedKey)
	}
}
