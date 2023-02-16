use std::collections::BTreeMap;

use openssl::{
    pkey::PKey,
    ssl::{SslContext, SslContextBuilder, SslMethod, SslVersion},
    x509::X509,
};

use crate::{Result, TlsConfig};

pub fn set_context_builder(builder: &mut SslContextBuilder, pkey: &str, cert: &str) -> Result<()> {
    let pkey = PKey::private_key_from_pem(pkey.as_bytes())?;
    builder.set_private_key(&pkey)?;

    let cert = X509::from_pem(cert.as_bytes())?;
    builder.set_certificate(&cert)?;
    // builder.set_min_proto_version(Some(SslVersion::TLS1))?;

    Ok(())
}

pub fn build_ssl_context(pkey: &str, cert: &str) -> Result<SslContext> {
    let mut builder = SslContextBuilder::new(SslMethod::tls())?;

    set_context_builder(&mut builder, pkey, cert)?;

    let ctx = builder.build();

    Ok(ctx)
}

pub fn build_ssl_context_map(configs: Vec<TlsConfig>) -> Result<BTreeMap<String, SslContext>> {
    let mut res = BTreeMap::new();

    for tls in configs {
        let ctx = build_ssl_context(&tls.private_key, &tls.certificate)?;
        res.insert(tls.sni, ctx);
    }
    Ok(res)
}
