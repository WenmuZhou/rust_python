#[macro_use] extern crate cpython;
use cpython::{PyResult, Python};
use std::mem;

// 生成可以直接在python中import的so
// 此处的module_name 和 initmodule_name可以随意命名
// PyInit_edit_distence_rust1 这里的edit_distence_rust1要和使用时的so的文件名一致
py_module_initializer!(module_name, initmodule_name, PyInit_edit_distence_rust, |py, m| {
    m.add(py, "__doc__", "Module documentation string")?;
    m.add(py, "edit_distance", py_fn!(py, edit_distance_py(a: &str, b: &str)))?;
    Ok(())
});
fn edit_distance_py(_: Python, a: &str, b: &str) -> PyResult<i32> {
    let result = edit_distance(a,b) as i32;
    Ok(result)
}

pub fn edit_distance(a: &str, b: &str) -> usize {
    let mut a = a;
    let mut b = b;
    let mut len_a = a.chars().count();
    let mut len_b = b.chars().count();
    if len_a < len_b{
        mem::swap(&mut a, &mut b);
        mem::swap(&mut len_a, &mut len_b);
    }
    // handle special case of 0 length
    if len_a == 0 {
        return len_b
    } else if len_b == 0 {
        return len_a
    }

    let len_b = len_b + 1;

    let mut pre;
    let mut tmp;
    let mut cur = vec![0; len_b];

    // initialize string b
    for i in 1..len_b {
        cur[i] = i;
    }

    // calculate edit distance
    for (i,ca) in a.chars().enumerate() {
        // get first column for this row
        pre = cur[0];
        cur[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            tmp = cur[j + 1];
            cur[j + 1] = std::cmp::min(
                // deletion
                tmp + 1, std::cmp::min(
                // insertion
                cur[j] + 1,
                // match or substitution
                pre + if ca == cb { 0 } else { 1 }));
            pre = tmp;
        }
    }
    cur[len_b - 1]
}

// 生成单独的so文件，python中使用ctypes调用
use std::os::raw::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn edit_distance_so(a: *const c_char, b: *const c_char) -> usize {
    let a = unsafe { CStr::from_ptr(a) };
    let mut a: &str =a.to_str().unwrap(); //
    let b = unsafe { CStr::from_ptr(b) };
    let mut b: &str = b.to_str().unwrap(); //

    let mut len_a = a.chars().count();
    let mut len_b = b.chars().count();
    if len_a < len_b{
        mem::swap(&mut a, &mut b);
        mem::swap(&mut len_a, &mut len_b);
    }
    // handle special case of 0 length
    if len_a == 0 {
        return len_b
    } else if len_b == 0 {
        return len_a
    }

    let len_b = len_b + 1;

    let mut pre;
    let mut tmp;
    let mut cur = vec![0; len_b];

    // initialize string b
    for i in 1..len_b {
        cur[i] = i;
    }

    // calculate edit distance
    for (i,ca) in a.chars().enumerate() {
        // get first column for this row
        pre = cur[0];
        cur[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            tmp = cur[j + 1];
            cur[j + 1] = std::cmp::min(
                // deletion
                tmp + 1, std::cmp::min(
                // insertion
                cur[j] + 1,
                // match or substitution
                pre + if ca == cb { 0 } else { 1 }));
            pre = tmp;
        }
    }
    cur[len_b - 1]
}