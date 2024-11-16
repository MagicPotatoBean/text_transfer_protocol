use std::{net::{TcpStream, ToSocketAddrs}, path::PathBuf};

pub struct TtpRequest {
    /// The domain should be a vector of labels i.e. instead of ["test.com"], use ["test", "com"].
    domain: Vec<String>,
    method: TtpMethod,
}
impl TtpRequest {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) {
        let mut buffer: Vec<u8> = Vec::new();
        use std::io::Write;
        match &self.method {
            TtpMethod::Query { paths } => {
                for path in paths {
                    // Query method indicator
                    buffer.write_all(&[0]).expect("Writing to Vec<u8> shouldnt fail");
                    // Write labels
                    for label in self.domain.iter() {
                        // Label length indicator
                        buffer.write_all(&[label.len() as u8]).expect("Writing to Vec<u8> shouldnt fail");
                        // Label value
                        buffer.write_all(label.as_bytes()).expect("Writing to Vec<u8> shouldnt fail");
                    }
                    // Zero-length end label, path-length and body length (0)
                    buffer.write_all(&[0, path.len() as u8, 0, 0]).expect("Writing to Vec<u8> shouldnt fail");
                }
            },
            TtpMethod::UploadReplace { path, body } => {
                // UploadReplace method indicator
                buffer.write_all(&[1]).expect("Writing to Vec<u8> shouldnt fail");
                // Write labels
                for label in self.domain.iter() {
                    // Label length indicator
                    buffer.write_all(&[label.len() as u8]).expect("Writing to Vec<u8> shouldnt fail");
                    // Label value
                    buffer.write_all(label.as_bytes()).expect("Writing to Vec<u8> shouldnt fail");
                }
                let body_len = body.len();
                let body_len_len = body.
                // Zero-length end label, path-length and body length (0)
                buffer.write_all(&[0, path.len() as u8, 0, 0]).expect("Writing to Vec<u8> shouldnt fail");
            },
            TtpMethod::UploadUnique { path, body } => {
                // Query method indicator
                buffer.write_all(&[0]).expect("Writing to Vec<u8> shouldnt fail");
                // Write labels
                for label in self.domain.iter() {
                    // Label length indicator
                    buffer.write_all(&[label.len() as u8]).expect("Writing to Vec<u8> shouldnt fail");
                    // Label value
                    buffer.write_all(label.as_bytes()).expect("Writing to Vec<u8> shouldnt fail");
                }
                // Zero-length end label, path-length and body length (0)
                buffer.write_all(&[0, path.len() as u8, 0, 0]).expect("Writing to Vec<u8> shouldnt fail");
            },
        }
    }
}
pub enum TtpMethod {
    Query{paths: Vec<String>},
    UploadReplace{path: String, body: Vec<u8>},
    UploadUnique{path: String, body: Vec<u8>},
}

pub fn request<A: ToSocketAddrs>(addr: A) -> std::io::Result<Vec<u8>> {
    let stream = TcpStream::connect(addr)?;
    todo!()
}
