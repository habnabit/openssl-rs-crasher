fn main() -> Result<(), openssl::error::ErrorStack> {
    let key_bytes = b"\
-----BEGIN PUBLIC KEY-----\n\
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAwVACPi9w23mF3tBkdZz+\n\
zwrzKOaaQdr01vAbU4E1pvkfj4sqDsm6lyDONS789sVoD/xCS9Y0hkkC3gtL1tSf\n\
TlgCMOOul9lcixlEKzwKENj1Yz/s7daSan9tqw3bfUV/nqgbhGX81v/+7RFAEd+R\n\
wFnK7a+XYl9sluzHRyVVaTTveB2GazTwEfzk2DWgkBluml8OREmvfraX3bkHZJTK\n\
X4EQSjBbbdJ2ZXIsRrYOXfaA+xayEGB+8hdlLmAjbCVfaigxX0CDqWeR1yFL9kwd\n\
9P0NsZRPsmoqVwMbMu7mStFai6aIhc3nSlv8kg9qv1m6XHVQY3PnEw+QQtqSIXkl\n\
HwIDAQAB\n\
-----END PUBLIC KEY-----";
    let key = openssl::rsa::Rsa::public_key_from_pem(key_bytes)?;
    let v = key.n().to_vec();
    println!("n bytes: {}", v.len());
    Ok(())
}
