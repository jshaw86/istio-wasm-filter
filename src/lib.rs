use log::info;
use proxy_wasm as wasm;

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
    fn on_http_response_headers(&mut self, num_headers: usize, _end_of_stream: bool) -> wasm::types::Action {
        info!("Got {} HTTP headers in #{}.", num_headers, self.context_id);
        self.set_http_response_header("x-hello", Some("here"));

        wasm::types::Action::Continue
    }
}
