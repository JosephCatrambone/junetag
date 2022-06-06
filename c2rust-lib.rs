#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(register_tool)]
#![register_tool(c2rust)]


extern crate libc;
#[path = "../example/apriltag_demo.rs"]
pub mod apriltag_demo;
#[path = "../apriltag.rs"]
pub mod apriltag;
#[path = "../apriltag_pose.rs"]
pub mod apriltag_pose;
#[path = "../apriltag_quad_thresh.rs"]
pub mod apriltag_quad_thresh;
#[path = "../common/g2d.rs"]
pub mod g2d;
#[path = "../common/getopt.rs"]
pub mod getopt;
#[path = "../common/homography.rs"]
pub mod homography;
#[path = "../common/image_u8.rs"]
pub mod image_u8;
#[path = "../common/image_u8x3.rs"]
pub mod image_u8x3;
#[path = "../common/image_u8x4.rs"]
pub mod image_u8x4;
#[path = "../common/matd.rs"]
pub mod matd;
#[path = "../common/pam.rs"]
pub mod pam;
#[path = "../common/pjpeg_idct.rs"]
pub mod pjpeg_idct;
#[path = "../common/pjpeg.rs"]
pub mod pjpeg;
#[path = "../common/pnm.rs"]
pub mod pnm;
#[path = "../common/string_util.rs"]
pub mod string_util;
#[path = "../common/svd22.rs"]
pub mod svd22;
#[path = "../common/time_util.rs"]
pub mod time_util;
#[path = "../common/unionfind.rs"]
pub mod unionfind;
#[path = "../common/workerpool.rs"]
pub mod workerpool;
#[path = "../common/zarray.rs"]
pub mod zarray;
#[path = "../common/zhash.rs"]
pub mod zhash;
#[path = "../common/zmaxheap.rs"]
pub mod zmaxheap;
#[path = "../tag16h5.rs"]
pub mod tag16h5;
#[path = "../tag25h9.rs"]
pub mod tag25h9;
#[path = "../tag36h10.rs"]
pub mod tag36h10;
#[path = "../tag36h11.rs"]
pub mod tag36h11;
#[path = "../tagCircle21h7.rs"]
pub mod tagCircle21h7;
#[path = "../tagCircle49h12.rs"]
pub mod tagCircle49h12;
#[path = "../tagCustom48h12.rs"]
pub mod tagCustom48h12;
#[path = "../tagStandard41h12.rs"]
pub mod tagStandard41h12;
#[path = "../tagStandard52h13.rs"]
pub mod tagStandard52h13;
