use counter::singleton;
use server::Request;
use server::RequestHelper;
use server::Response;
use server::ResponseHelper;
use server::METHOD;

pub extern "C" fn execute_wrapper(request: *mut Request, response: *mut Response) -> i32 {
    let method = request.get_method();
    match method {
        METHOD::GET => {
            let rr = request.get_path();
            let path = rr.as_ref();
            match &*path {
                "/" => response.set_response_body("root".as_ref()),
                "/count" => {
                    let s = singleton();
                    let mut data = s.inner.lock().unwrap();
                    response.set_response_body(format!("count:{}", *data).as_ref());
                }
                _ => {
                    response.set_status(404);
                    response.set_response_body("Not Found".as_ref())
                }
            }
        }
        METHOD::POST => {
            let rr = request.get_path();
            let path = rr.as_ref();
            match &*path {
                "/" => response.set_response_body("root".as_ref()),
                "/count" => {
                    let s = singleton();
                    let mut data = s.inner.lock().unwrap();
                    response.set_response_body(format!("count:{}", *data).as_ref());
                    *data = *data + 1;
                }
                "/stop" => {
                    response.set_stop(1);
                }
                _ => {
                    response.set_status(404);
                    response.set_response_body("Not Found".as_ref())
                }
            }
        }
    }

    let ret = 1;
    ret
}
