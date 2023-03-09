use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(HttpAuthRandom) } );
}}

struct HttpAuthRandom;

impl HttpContext for HttpAuthRandom {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        self.dispatch_http_call(
            "httpbin", 
            vec![
                (":method", "GET"),
                (":path", "/bytes/1"),
                (":authority", "httpbin.org"),
            ], 
            None, 
            vec![], 
            Duration::from_secs(5),
        )
        .unwrap();
        Action::Pause
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        self.set_http_response_header("Powered-By", Some("proxy-wasm"));
        Action::Continue
    }
}

impl Context for HttpAuthRandom {
    fn on_http_call_response(
            &mut self,
            _token_id: u32,
            _num_headers: usize,
            body_size: usize,
            _num_trailers: usize,
        ) {
        if let Some(body) = self.get_http_call_response_body(0, body_size) {
            if !body.is_empty() && body[0] % 2 == 0 {
                info!("Access granted.");
                self.resume_http_request();
                return;
            }
        }

        info!("Access forbidden.");
        self.send_http_response(
            403, 
            vec![("Powered-By", "proxy-wasm")],
            Some(b"Access forbidden.\n")
        );
    }
}
