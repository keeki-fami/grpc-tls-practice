use rcgen::{BasicConstraints, Certificate, CertificateParams, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair, KeyUsagePurpose};
use rcgen::DnValue::PrintableString;
use std::fs;
use time::{OffsetDateTime, Duration};

fn main() {
    // read self-signed certificate
    // create issuer
    // generate certificate for server/client

    let issuer = setup_issuer();
    let cert = generate_new_cert(&issuer, "iPhoneClient");

    fs::write(format!("{}.pem", "iPhoneClient"), cert.pem()).expect("failed to write privatekey");
}

fn setup_issuer() -> Issuer<'static, KeyPair> {
    // if `aws_lc_rs` features is used, then the key must be a DER-encoded plaintext private key
    // let key = fs::read_to_string("./key/root-ca.key.pem").expect("rootCA error: fail to read private key");
    let cert = fs::read_to_string("./cert/root-ca.pem").expect("rootCA error: fail to read certificate");
    let key = fs::read_to_string("./key/root-ca.key.pem").expect("rootCA error: fail to read private key");
    let key_pair = KeyPair::from_pem(key.as_str()).expect("rootCA error: failed to get KeyPair");
    let issuer = Issuer::from_ca_cert_pem(cert.as_str(), key_pair).expect("rootCA error: failed to get Issuer");
    issuer
}

fn generate_new_cert(
    issuer: &Issuer<'static, KeyPair>,
    name: &str
 ) -> Certificate {
    let mut params = CertificateParams::new(vec![name.into()]).expect("we know the name is valid");
    // let (before, after) 証明書の期限を設定

    params.distinguished_name.push(DnType::CommonName, name);

    // これをやる意味？
    // params.use_authority_key_identifier_extension = true;

    params.key_usages.push(KeyUsagePurpose::DigitalSignature);

    params.extended_key_usages.push(ExtendedKeyUsagePurpose::ServerAuth);

    // 今回は作らない。ファイルから取得する。あるいは、コマンドラインから渡す。
    // 秘密鍵も渡すの？
    let key_pair = KeyPair::generate().unwrap();

    fs::write(format!("{}.key.pem", name), key_pair.serialize_pem()).expect("failed to write privatekey");

    let day = Duration::new(86400*30, 0);
    let offsetDateTime = OffsetDateTime::now_utc();
    let now = offsetDateTime;
    let limit = offsetDateTime.checked_add(day).unwrap();

    // 証明書の期間を設定
    params.not_before = now;
    params.not_after = limit;

    // key_pairには、証明書の発行を依頼したクライアント/サーバの公開鍵
    // issuerには、署名者の情報を渡す。
    params.signed_by(&key_pair, issuer).unwrap()
}