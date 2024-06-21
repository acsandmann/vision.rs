#![deny(clippy::all)]
#![allow(improper_ctypes, non_camel_case_types, non_upper_case_globals)]

#[macro_use]
extern crate objc;

use std::ffi::CStr;

use cocoa::{
  base::nil,
  foundation::{NSArray, NSAutoreleasePool, NSString},
};
use core_graphics2::{
  color_space::CGColorRenderingIntent,
  data_provider::CGDataProvider,
  image::{CGImage, __CGImage},
};
use ffi::{
  VNImageRequestHandler, VNRecognizeTextRequest, VNRecognizedText, VNRecognizedTextObservation,
};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

mod ffi;

#[napi]
pub struct ocr {}

#[napi]
impl ocr {
  #[napi]
  pub unsafe fn process(b: Buffer, t: Option<u8>) -> Vec<&'static str> {
    let mut res: Vec<&str> = vec![];
    let data_provider = CGDataProvider::from_slice(b.as_ref()).unwrap();
    let image = match t {
      None => CGImage::from_png_data_provider(
        &data_provider,
        None,
        false,
        CGColorRenderingIntent::Default,
      )
      .unwrap(),
      Some(c) => match c {
        0 => CGImage::from_jpeg_data_provider(
          &data_provider,
          None,
          false,
          CGColorRenderingIntent::Default,
        )
        .unwrap(),
        _ => CGImage::from_png_data_provider(
          &data_provider,
          None,
          false,
          CGColorRenderingIntent::Default,
        )
        .unwrap(),
      },
    };
    let raw_ptr: *mut __CGImage =
      std::mem::transmute::<CGImage, *mut CGImage>(image) as *mut __CGImage;
    let handler = VNImageRequestHandler::alloc(nil)
      .initWithCGImage(raw_ptr)
      .autorelease();
    let request = VNRecognizeTextRequest::alloc(nil);
    VNRecognizeTextRequest::init(request).autorelease();
    handler.performRequests(&[request], None);
    let results = request.results();
    for i in 0..results.count() {
      let s = results.objectAtIndex(i).topCandidates(1);
      for j in 0..s.count() {
        let raw_string = s.objectAtIndex(j).string();
        res.push(
          CStr::from_ptr(raw_string.UTF8String())
            .to_str()
            .unwrap_unchecked(),
        );
      }
    }
    res
  }
}

#[napi]
pub unsafe fn vn_recognize_text_request(b: Buffer, t: Option<u8>) -> Vec<String> {
  let mut res: Vec<String> = vec![];
  let data_provider = CGDataProvider::from_slice(b.as_ref()).unwrap();
  let image = match t {
    None => {
      CGImage::from_png_data_provider(&data_provider, None, false, CGColorRenderingIntent::Default)
        .unwrap()
    }
    Some(c) => match c {
      0 => CGImage::from_jpeg_data_provider(
        &data_provider,
        None,
        false,
        CGColorRenderingIntent::Default,
      )
      .unwrap(),
      _ => CGImage::from_png_data_provider(
        &data_provider,
        None,
        false,
        CGColorRenderingIntent::Default,
      )
      .unwrap(),
    },
  };
  let raw_ptr: *mut __CGImage =
    std::mem::transmute::<CGImage, *mut CGImage>(image) as *mut __CGImage;
  let handler = VNImageRequestHandler::alloc(nil)
    .initWithCGImage(raw_ptr)
    .autorelease();
  let request = VNRecognizeTextRequest::alloc(nil);
  VNRecognizeTextRequest::init(request).autorelease();
  handler.performRequests(&[request], None);
  let results = request.results();
  for i in 0..results.count() {
    let s = results.objectAtIndex(i).topCandidates(1);
    for j in 0..s.count() {
      let raw_string = s.objectAtIndex(j).string();
      res.push(
        String::from_utf8_lossy(CStr::from_ptr(raw_string.UTF8String()).to_bytes()).to_string(),
      );
    }
  }
  res
}
