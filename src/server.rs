extern crate libc;

use std::ffi::CStr;
use std::mem;

pub enum Context {}

#[repr(C)]
pub struct Response{
	pub status: i8,
	pub response_body: *mut libc::c_char
}
pub trait ResponseHelper {
	fn set_response_body(self, body: &str);
<<<<<<< HEAD
	fn set_status(self, status: i32);
=======
	fn set_status(self, status: i8);
>>>>>>> origin/master
}
impl ResponseHelper for *mut Response {
	fn set_response_body(self, body: &str) {
		unsafe {
			let ret: *mut libc::c_char = mem::transmute(body.as_ptr());
			response_set_response_body(self, ret);
		}
	}
<<<<<<< HEAD
	fn set_status(self, status: i32) {
=======
	fn set_status(self, status: i8) {
>>>>>>> origin/master
		unsafe { response_set_status(self, status) };
	}
}

#[repr(C)]
#[derive(Debug)]
pub enum METHOD {GET, POST}

#[repr(C)]
pub struct Request {
	pub method: METHOD,
	pub path: *mut libc::c_char,
	pub query: *mut libc::c_char,
	pub request_body: *mut libc::c_char
}

pub trait RequestHelper {
	fn get_method(self) -> METHOD;
	fn get_path(self) -> String;
	fn get_query(self) -> String;
	fn get_request_body(self) -> String;
}

fn convert_string(source: *const libc::c_char) -> String {
	let c_str: &CStr = unsafe { CStr::from_ptr(source) };
	let str = c_str.to_str().unwrap().to_owned();
	str
}

impl RequestHelper for *mut Request {
	fn get_method(self) -> METHOD {
		return unsafe { request_get_method(self) };
	}
	fn get_path(self) -> String {
		return convert_string(unsafe { request_get_path(self) } );
	}
	fn get_query(self) -> String {
		return convert_string(unsafe { request_get_query(self) } );
	}
	fn get_request_body(self) -> String {
		return convert_string(unsafe { request_get_request_body(self) } );
	}
}

pub fn startup(callback: extern fn(*mut Request, *mut Response) -> i32) {
	unsafe {
		let ctx = ::server::new_context();
		context_set_callback(ctx, callback);
		serve(ctx);
	}
}

extern "C" {
	pub fn new_context() -> *mut Context;
	pub fn serve(context: *mut Context) -> i32;
	pub fn context_set_callback(context: *const Context, callback: extern fn(*mut Request, *mut Response) -> i32) -> *mut libc::c_void;

<<<<<<< HEAD
	pub fn response_set_status(request: *mut Response, status:i32);
=======
	pub fn response_set_status(request: *mut Response, status:i8);
>>>>>>> origin/master
	pub fn response_set_response_body(request: *mut Response, body: *const libc::c_char);

	pub fn request_get_method(request: *mut Request) -> METHOD;
	pub fn request_get_path(request: *mut Request) -> *mut libc::c_char;
	pub fn request_get_query(request: *mut Request) -> *mut libc::c_char;
	pub fn request_get_request_body(request: *mut Request) -> *mut libc::c_char;

}
