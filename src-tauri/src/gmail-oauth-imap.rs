extern crate base64;
extern crate imap;

struct GmailOAuth2 {
    user: String,
    access_token: String,
}

impl imap::Authenticator for GmailOAuth2 {
    type Response = String;
    #[allow(unused_variables)]
    fn process(&self, data: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        )
    }
}

fn main() {
    let gmail_auth = GmailOAuth2 {
        user: String::from("mcmadafly@gmail.com"),
        access_token: String::from("ya29.a0AcM612yb3Ylff2l6T6JnDJ8s_2n0QcfeNLj4FS_2WYKg3oQvQ_Z-L0Y5k1IKw29QUVX6anoONVuiYGj1iEGTCDUNuR8RsPcdgA9TACVm2AKXHhNJQNGoyUdxw4uooFtaavj9xjo3lNhApqz5qJy9HXUDhL44J-6ESvrpOvNVaCgYKAaMSARESFQHGX2MiR0xv3RHP2dFbt4HnmMfNsQ0175"),
    };


    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect(("imap.gmail.com", 993), "imap.gmail.com", &tls)
        .expect("Could not connect to imap.gmail.com");
    
    let mut imap_session = match client.authenticate("XOAUTH2", &gmail_auth) {
        Ok(c) => c,
        Err((e, _unauth_client)) => {
            println!("error authenticating: {}", e);
            return;
        }
    };

    match imap_session.select("INBOX") {
        Ok(mailbox) => println!("{}", mailbox),
        Err(e) => println!("Error selecting INBOX: {}", e),
    };

    match imap_session.fetch("1", "") {
        Ok(msgs) => {
            for msg in msgs.iter() {
                print!("{:?}", msg);
            }
        }
        Err(e) => println!("Error Fetching email 2: {}", e),
    };

    imap_session.logout().unwrap();
}