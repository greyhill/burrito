extern crate libc;

use libc::*;
use std::slice;
use std::convert::{From, Into};
use std::mem::transmute;
use std::iter::repeat;

#[macro_export]
macro_rules! mex_entry {
    ( $func: ident ) => {
        use std::slice;

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn mexFunction(nlhs: i32, lhs: *mut Array,
                           nrhs: i32, rhs: *const Array) -> () {
            let lhs_slice = unsafe { slice::from_raw_parts_mut(lhs, nlhs as usize) };
            let rhs_slice = unsafe { slice::from_raw_parts(rhs, nrhs as usize) };
            $func(lhs_slice, rhs_slice);
        }
    }
}

pub struct Array {
    ptr: *mut c_void,
}

impl Array {
    pub fn new(dims: Vec<usize>, class: MxClassID, complex: bool) -> Array {
        let ndims_mwsize = dims.len() as MwSize;
        let dims_mwsize: Vec<MwSize> = dims.iter().map(|&x| x as MwSize).collect();
        Array{
            ptr: unsafe { mxCreateNumericArray(ndims_mwsize,
                                               dims_mwsize.as_ptr(),
                                               class.into(),
                                               if complex { 1i32 } else { 0i32 }) }
        }
    }

    pub fn get_class(self: &Self) -> MxClassID {
        MxClassID::from(unsafe { mxGetClassID(self.ptr) })
    }

    pub fn get_dimensions(self: &Self) -> Vec<usize> {
        let num_dims = unsafe { mxGetNumberOfDimensions(self.ptr) } as usize;
        let dims = unsafe { mxGetDimensions(self.ptr) };
        let dims_slice = unsafe {
            slice::from_raw_parts(dims, num_dims)
        };
        dims_slice.iter().map(|&x| x as usize).collect()
    }

    pub fn set_dimensions(self: &mut Self, dims: Vec<usize>) -> () {
        let ndims_mwsize = dims.len() as MwSize;
        let dims_mwsize: Vec<MwSize> = dims.iter().map(|&x| x as MwSize).collect();
        unsafe { mxSetDimensions(self.ptr, dims_mwsize.as_ptr(),
                                           ndims_mwsize); }
    }

    pub fn real_data(self: &Self) -> Data {
        let ptr = unsafe { mxGetData(self.ptr) };
        let dims = self.get_dimensions();
        let numel = dims.iter().fold(1usize, |l,r| l*r);
        match self.get_class() {
            MxClassID::Bool => Data::Bool(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),

            MxClassID::F64 => Data::F64(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),
            MxClassID::F32 => Data::F32(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),

            MxClassID::I8 => Data::I8(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),
            MxClassID::U8 => Data::U8(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),

            MxClassID::I16 => Data::I16(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),
            MxClassID::U16 => Data::U16(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),

            MxClassID::I32 => Data::I32(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),
            MxClassID::U32 => Data::U32(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),

            MxClassID::I64 => Data::I64(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),
            MxClassID::U64 => Data::U64(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),

            _ => Data::NotNumeric
        }
    }

    pub fn imag_data(self: &Self) -> Data {
        let ptr = unsafe { mxGetImagData(self.ptr) };
        let dims = self.get_dimensions();
        let numel = dims.iter().fold(1usize, |l,r| l*r);
        match self.get_class() {
            MxClassID::Bool => Data::Bool(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),

            MxClassID::F64 => Data::F64(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),
            MxClassID::F32 => Data::F32(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),

            MxClassID::I8 => Data::I8(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),
            MxClassID::U8 => Data::U8(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),

            MxClassID::I16 => Data::I16(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),
            MxClassID::U16 => Data::U16(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),

            MxClassID::I32 => Data::I32(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),
            MxClassID::U32 => Data::U32(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),

            MxClassID::I64 => Data::I64(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),
            MxClassID::U64 => Data::U64(unsafe { slice::from_raw_parts(transmute(ptr), numel) }),

            _ => Data::NotNumeric
        }
    }

    pub fn real_data_mut(self: &mut Self) -> DataMut {
        let ptr = unsafe { mxGetData(self.ptr) };
        let dims = self.get_dimensions();
        let numel = dims.iter().fold(1usize, |l,r| l*r);
        match self.get_class() {
            MxClassID::Bool => DataMut::Bool(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),

            MxClassID::F64 => DataMut::F64(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),
            MxClassID::F32 => DataMut::F32(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),

            MxClassID::I8 => DataMut::I8(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),
            MxClassID::U8 => DataMut::U8(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),

            MxClassID::I16 => DataMut::I16(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),
            MxClassID::U16 => DataMut::U16(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),

            MxClassID::I32 => DataMut::I32(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),
            MxClassID::U32 => DataMut::U32(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),

            MxClassID::I64 => DataMut::I64(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),
            MxClassID::U64 => DataMut::U64(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),

            _ => DataMut::NotNumeric
        }
    }

    pub fn imag_data_mut(self: &mut Self) -> DataMut {
        let ptr = unsafe { mxGetImagData(self.ptr) };
        let dims = self.get_dimensions();
        let numel = dims.iter().fold(1usize, |l,r| l*r);
        match self.get_class() {
            MxClassID::Bool => DataMut::Bool(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),

            MxClassID::F64 => DataMut::F64(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),
            MxClassID::F32 => DataMut::F32(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),

            MxClassID::I8 => DataMut::I8(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),
            MxClassID::U8 => DataMut::U8(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),

            MxClassID::I16 => DataMut::I16(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),
            MxClassID::U16 => DataMut::U16(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),

            MxClassID::I32 => DataMut::I32(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),
            MxClassID::U32 => DataMut::U32(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),

            MxClassID::I64 => DataMut::I64(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),
            MxClassID::U64 => DataMut::U64(unsafe { slice::from_raw_parts_mut(transmute(ptr), numel) }),

            _ => DataMut::NotNumeric
        }
    }

    pub fn as_string(self: &Self) -> String {
        let dims = self.get_dimensions();
        let numel = dims.iter().fold(1usize, |l,r| l*r);
        let mut buf: Vec<u8> = repeat(0u8).take(numel+1).collect();
        unsafe {
            mxGetString(self.ptr, 
                        transmute(buf.as_mut_ptr()),
                        numel as MwSize + 1);
        }
        buf.pop(); // remove trailing \0
        String::from_utf8(buf).ok().unwrap()
    }
}

pub enum DataMut<'a> {
    NotNumeric,

    Bool(&'a mut [bool]),

    F64(&'a mut [f64]),
    F32(&'a mut [f32]),

    I8(&'a mut [i8]),
    U8(&'a mut [u8]),

    I16(&'a mut [i16]),
    U16(&'a mut [u16]),

    I32(&'a mut [i32]),
    U32(&'a mut [u32]),

    I64(&'a mut [i64]),
    U64(&'a mut [u64]),
}

pub enum Data<'a> {
    NotNumeric,

    Bool(&'a [bool]),

    F64(&'a [f64]),
    F32(&'a [f32]),

    I8(&'a [i8]),
    U8(&'a [u8]),

    I16(&'a [i16]),
    U16(&'a [u16]),

    I32(&'a [i32]),
    U32(&'a [u32]),

    I64(&'a [i64]),
    U64(&'a [u64]),
}

#[cfg(octave)]
pub type MwSize = u32;

#[cfg(matlab)]
pub type MwSize = usize;

#[derive(Debug, Copy, Clone)]
pub enum MxClassID {
    Unknown,
    Cell,
    Struct,
    Bool,
    Char,
    Unused,
    F64,
    F32,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    Function
}

impl From<i32> for MxClassID {
    fn from(id: i32) -> MxClassID {
        use MxClassID::*;
        match id {
            0 => Unknown,
            1 => Cell,
            2 => Struct,
            3 => Bool,
            4 => Char,
            5 => Unused,
            6 => F64,
            7 => F32,
            8 => I8,
            9 => U8,
            10 => I16,
            11 => U16,
            12 => I32,
            13 => U32,
            14 => I64,
            15 => U64,
            16 => Function,
            _ => Unknown // or error?
        }
    }
}

impl Into<i32> for MxClassID {
    fn into(self: Self) -> i32 {
        use MxClassID::*;
        match self {
            Unknown => 0,
            Cell => 1,
            Struct => 2,
            Bool => 3,
            Char => 4,
            Unused => 5,
            F64 => 6,
            F32 => 7,
            I8 => 8,
            U8 => 9,
            I16 => 10,
            U16 => 11,
            I32 => 12,
            U32 => 13,
            I64 => 14,
            U64 => 15,
            Function => 16
        }
    }
}

#[cfg(octave)]
#[link(name = "octave")]
#[link(name = "octinterp")]
extern { }

#[cfg(matlab)]
#[link(name = "mex")]
#[link(name = "mat")]
extern { }

extern {
    // type checks
    fn mxGetClassID(ptr: *const c_void) -> i32;

    // creation
    fn mxCreateNumericArray(ndim: MwSize, dims: *const MwSize,
                            class: i32, complex: i32) -> *mut c_void;

    // dimension interrogation
    fn mxGetDimensions(ptr: *const c_void) -> *const MwSize;
    fn mxGetNumberOfDimensions(ptr: *const c_void) -> MwSize;

    // dimension setting
    fn mxSetDimensions(ptr: *mut c_void, dims: *const MwSize, ndims: MwSize) -> ();

    // data extractors
    fn mxGetData(ptr: *const c_void) -> *mut c_void;
    fn mxGetImagData(ptr: *const c_void) -> *mut c_void;
    fn mxGetString(ptr: *const c_void, s: *mut char, strlen: MwSize) -> i32;
}

