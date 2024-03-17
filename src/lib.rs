use log::info;
use proxy_wasm as wasm;
use proxy_wasm::traits::Context;
use std::str;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(wasm::types::LogLevel::Trace);
    proxy_wasm::set_http_context(
        |context_id, _root_context_id| -> Box<dyn wasm::traits::HttpContext> {
            Box::new(HelloWorld { context_id })
        },
    )
}

struct HelloWorld {
    context_id: u32,
}

impl wasm::traits::Context for HelloWorld {}

impl wasm::traits::HttpContext for HelloWorld {

    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> wasm::types::Action {
        let headers = self.get_http_request_headers();

        for (name, value) in &headers {
            info!("name {} value {}", name, value)
        } 

        wasm::types::Action::Continue
    }


    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> wasm::types::Action {
        let prop = self.get_property(["source", "address"].to_vec()).unwrap();
        let addr = match str::from_utf8(&prop) {
            Ok(v) => v,
            Err(_e) => "",
        };
       
        //let resp = reqwest::blocking::get("http://eu.httpbin.org/ip")?
        //      .json::<HashMap<String, String>>()?;

        self.set_http_response_header("x-hello", Some(&format!("Hello world from v0.5 {}", self.context_id)));
        self.set_http_response_header("x-ip", Some(&addr));

        wasm::types::Action::Continue
    }
}
