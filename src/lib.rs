use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn rust_fibo(n: *mut c_int) -> *mut c_int {
    if n <= 1 as *mut i32 {
        n
    } else {
        let x = n as i32 - 2;
        let y = n as i32 - 1;
        (rust_fibo(x as *mut i32) as i32 + rust_fibo(y as *mut i32) as i32) as *mut i32
    }
}

#[cfg(target_os = "android")]
pub mod android {
    extern crate jni;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jint, jstring};
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_totechite_rustpractice_RustGreetings_greeting(
        env: JNIEnv,
        _: JClass,
        input: JString,
    ) -> jstring {
        let input: String = env
            .get_string(input)
            .expect("invalid pattern string")
            .into();
        let output = env
            .new_string(format!("Hello!. I\'m {}", input))
            .expect("Couldn't create java string!");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_totechite_rustpractice_RustGreetings_fibo(
        _env: JNIEnv,
        _: JClass,
        java_int: jint,
    ) -> jint {
        let fibo = rust_fibo(java_int as *mut i32);
        fibo as jint
    }
}
