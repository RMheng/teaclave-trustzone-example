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

use optee_teec::{Context, Operation, ParamType, Session, Uuid};
use optee_teec::{ParamNone, ParamValue, ParamTmpRef};
use proto::{UUID, Command};

fn data_compute(session: &mut Session) -> optee_teec::Result<()> {
    let nums1:[u8; 5] = [1, 2, 3, 4, 5]; 
    let nums2:[u8; 5] = [4, 5, 6, 7, 8];
    //let mut vec_intersection = [0; 5];
    //let mut vec_union = [0; 10];
    let mut resu = [0; 10];
    let p1 = ParamTmpRef::new_input(&nums1);
    let p2 = ParamTmpRef::new_input(&nums2);
    let p3 = ParamTmpRef::new_output(&mut resu);
    //let p3 = ParamTmpRef::new_output(&mut vec_intersection);
    //let p4 = ParamTmpRef::new_output(&mut vec_union);
    let mut operation = Operation::new(0, p1, p2, p3, ParamNone);

    println!("intersection invoke");
    session.invoke_command(Command::Intersection as u32, &mut operation)?;
    //println!("Intersection resu = {:?}", &resu);
    
    println!("union invoke");
    session.invoke_command(Command::Union as u32, &mut operation)?;
    //println!("union resu =  {:?}", &resu);
    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    data_compute(&mut session)?;

    println!("Success");
    Ok(())
}
