use getopts::Options;
use std::env;
use tiny_http::{Server, StatusCode, Response};

struct CliArgs {
    host: String
}

fn print_usage(program: &str, opts: Options) {
    let usage = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&usage));
}

fn parse_arguments() -> Option<CliArgs> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("l", "host", "where the server should listen.\nDefault localhost:30210", "HOST");
    opts.optflag("h", "help", "print this message and exit");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e.to_string());
            print_usage(&program, opts);
            return None;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return None;
    }

    let host = match matches.opt_str("l") {
        Some(l) => l,
        None => "localhost:30210".to_owned()
    };

    Some(CliArgs {
        host
    })
}

fn main() {
    let args = match parse_arguments() {
        Some(a) => a,
        None => return
    };

    let server = match Server::http(args.host) {
        Ok(s) => s,
        Err(e) => {
            println!("Could not start server: {}", e);
            return;
        }
    };

    for request in server.incoming_requests() {
        // Drop the slash on the url
        let url = &request.url()[1..];
        let requested_status = match url.parse::<u16>() {
            Ok(n) => n,
            Err(_) => 404
        };

        let status_code = StatusCode::from(requested_status);
        let response = Response::from_string(status_code.default_reason_phrase()).with_status_code(status_code);
        request.respond(response).ok();
    }
}
