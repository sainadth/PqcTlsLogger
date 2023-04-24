extern crate syslog;

use std::collections::HashMap;
use syslog::{Facility, Formatter5424};

fn main() {
    let mut logger = PqcTlsLogger::default();
    logger.message = "asfsf".to_string();
    logger.log(LogLevel::Alert, logger.message.as_str());
    logger.log(LogLevel::Emergency, logger.message.as_str());
    logger.log(LogLevel::Critical, logger.message.as_str());
    logger.log(LogLevel::Error, logger.message.as_str());
    logger.log(LogLevel::Warning, logger.message.as_str());
    logger.log(LogLevel::Notice, logger.message.as_str());
    logger.log(LogLevel::Informational, logger.message.as_str());
    logger.log(LogLevel::Debug, logger.message.as_str());
}

struct PqcTlsLogger{
    message : String,
}

// impl Default for PqcTlsLogger{
//     fn default() -> PqcTlsLogger{
//         return PqcTlsLogger { message: String::new() }
//     }
// }

enum LogLevel{
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Informational,
    Debug,
}


impl PqcTlsLogger{
    fn default() -> PqcTlsLogger{
        return PqcTlsLogger { message: String::new() }
    }

    fn log(&self, _level : LogLevel, message: &str){
        let formatter = Formatter5424 {
            facility: Facility::LOG_USER,
            hostname: None,
            process: "[LOGGER]".into(),
            pid: 0,
        };
        match syslog::unix(formatter) {
            Err(e) => println!("impossible to connect to syslog: {:?}", e),
            Ok(mut writer) => {
                match _level{
                    LogLevel::Alert => writer.alert((1, HashMap::new(), message)).unwrap(),
                    LogLevel::Critical => writer.crit((1, HashMap::new(), message)).unwrap(),
                    LogLevel::Debug => writer.debug((1, HashMap::new(), message)).unwrap(),
                    LogLevel::Emergency => writer.emerg((1, HashMap::new(), message)).unwrap(),
                    LogLevel::Error => writer.err((1, HashMap::new(), message)).unwrap(),
                    LogLevel::Informational => writer.info((1, HashMap::new(), message)).unwrap(),
                    LogLevel::Notice => writer.notice((1, HashMap::new(), message)).unwrap(),
                    LogLevel::Warning => writer.warning((1, HashMap::new(), message)).unwrap()
                }
            }
        }
    }
}

/*
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}
*/