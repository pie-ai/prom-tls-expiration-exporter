use ssl_expiration::SslExpiration;

fn main() {
    let port = 443;
    let domain = "google.com";
    let expiration = SslExpiration::from_addr(format!("{}:{}", domain, port)).unwrap();
    println!("tls certificate for {}:{} expires in {} days", domain, port, expiration.days());
}
