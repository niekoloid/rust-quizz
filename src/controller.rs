use ::server::Request;
use ::server::Response;
use ::server::RequestHelper;
use ::server::ResponseHelper;
use ::server::METHOD;
use ::counter::Counter;
use ::counter::singleton;

pub extern "C" fn execute_wrapper(request: *mut Request, response: *mut Response) -> i32 {
    let method = request.get_method();
    match method {
        METHOD::GET => {
            let rr = request.get_path();
            let path = rr.as_ref();
            match &*path {
<<<<<<< HEAD
                "/" => response.set_response_body("root".as_ref()),
=======
                "/" => response.set_response_body("root"),
>>>>>>> origin/master
                "/count" => {
                    let s = singleton();
                    let mut data = s.inner.lock().unwrap();
                    response.set_response_body(format!("count:{}", *data).as_ref());
                },
                _ => {
                    response.set_status(404);
<<<<<<< HEAD
                    response.set_response_body("Not Found".as_ref())
=======
                    response.set_response_body("Not Found")
>>>>>>> origin/master
                },
            }
        },
        METHOD::POST => {
            let rr = request.get_path();
            let path = rr.as_ref();
            match &*path {
<<<<<<< HEAD
                "/" => response.set_response_body("root".as_ref()),
                "/count" => {
=======
                "/" => response.set_response_body("root"),
                " /count" => {
>>>>>>> origin/master
                    let s = singleton();
                    let mut data = s.inner.lock().unwrap();
                    response.set_response_body(format!("count:{}", *data).as_ref());
                    *data = *data + 1;
                },
                _ => {
                    response.set_status(404);
<<<<<<< HEAD
                    response.set_response_body("Not Found".as_ref())
=======
                    response.set_response_body("Not Found")
>>>>>>> origin/master
                },
            }
        }
    }

    let ret = 1;
    ret
}


