// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![no_main]

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use proto::Command;
use std::collections::HashSet;

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA open session");
    Ok(())
}

#[ta_close_session]
fn close_session() {
    trace_println!("[+] TA close session");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destroy");
}

#[ta_invoke_command]
fn invoke_command(cmd_id: u32, params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    let nums1 = unsafe { params.0.as_memref().unwrap().raw() };
    let nums2 = unsafe { params.1.as_memref().unwrap().raw() };
    let mut vec_resu = unsafe { params.2.as_memref().unwrap().raw() };
    
    let nums1_size = unsafe { (*nums1).size };
    let nums2_size = unsafe { (*nums2).size };
    match Command::from(cmd_id) {
        Command::Intersection => {
            let mut set: HashSet<u8> = HashSet::new();
            let mut vec_count = 0;
            for i in 0..nums1_size {
                let mut val_nums1 = 0;
                unsafe {
                    val_nums1 = *((*nums1).buffer as *mut u8).offset(i as isize);
                };
                set.insert(val_nums1);
            }
            
            for i in 0..nums2_size {
                let mut val_nums2 = 0;
                unsafe {
                    val_nums2 = *((*nums2).buffer as *mut u8).offset(i as isize);
                };
                
                if set.contains(&val_nums2) {
                    unsafe { *((*vec_resu).buffer as *mut u8).offset(vec_count as isize) = val_nums2; }
                    vec_count += 1;
                    set.remove(&val_nums2);
                }
            }
            unsafe{ (*vec_resu).size = vec_count; }
            Ok(())
        }
        Command::Union => {
            let mut set: HashSet<u8> = HashSet::new();
            let mut vec_count = 0;
            for i in 0..nums1_size {
                let mut val_nums1 = 0;
                unsafe {
                    val_nums1 = *((*nums1).buffer as *mut u8).offset(i as isize);
                    *((*vec_resu).buffer as *mut u8).offset(vec_count as isize) = val_nums1;
                }
                vec_count += 1;
                set.insert(val_nums1);
            }

            for i in 0..nums2_size {
                let mut val_nums2 = 0;
                unsafe {
                    val_nums2 = *((*nums2).buffer as *mut u8).offset(i as isize);
                };

                if !set.contains(&val_nums2) {
                    unsafe { *((*vec_resu).buffer as *mut u8).offset(vec_count as isize) = val_nums2; }
                    vec_count += 1;
                    set.insert(val_nums2);
                }
            }
            unsafe{ (*vec_resu).size = vec_count; }
            Ok(())
        }
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }
}

// TA configurations
const TA_FLAGS: u32 = 0;
const TA_DATA_SIZE: u32 = 32 * 1024;
const TA_STACK_SIZE: u32 = 2 * 1024;
const TA_VERSION: &[u8] = b"0.1\0";
const TA_DESCRIPTION: &[u8] = b"This is a hello world example.\0";
const EXT_PROP_VALUE_1: &[u8] = b"Hello World TA\0";
const EXT_PROP_VALUE_2: u32 = 0x0010;
const TRACE_LEVEL: i32 = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_FRAMEWORK_STACK_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
