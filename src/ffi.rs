#![allow(non_snake_case)]

use std::os::raw::c_void;

use cocoa::{
  base::{id, nil, BOOL},
  foundation::{NSArray, NSDictionary, NSRange, NSUInteger},
};
use core_graphics2::image::CGImageRef;
use objc::msg_send;

pub trait VNImageRequestHandler: Sized {
  unsafe fn alloc(_: Self) -> id {
    msg_send![class!(VNImageRequestHandler), alloc]
  }

  unsafe fn initWithCGImage(self, cgImage: CGImageRef) -> id;
  unsafe fn performRequests(self, requests: &[id], error: Option<*mut id /* NSError */>) -> BOOL;
}

impl VNImageRequestHandler for id {
  unsafe fn initWithCGImage(self, cgImage: CGImageRef) -> id {
    msg_send![
        self,
        initWithCGImage: cgImage as *const c_void
        options: NSDictionary::dictionary(nil)
    ]
  }

  unsafe fn performRequests(self, requests: &[id], error: Option<*mut id /* NSError */>) -> BOOL {
    msg_send![
        self,
        performRequests: NSArray::arrayWithObjects(nil, requests)
        error: error.unwrap_or(std::ptr::null_mut())
    ]
  }
}

pub trait VNRecognizeTextRequest: Sized {
  unsafe fn alloc(_: Self) -> id {
    msg_send![class!(VNRecognizeTextRequest), alloc]
  }

  unsafe fn init(self) -> id;
  unsafe fn results(self) -> id /* NSArray */;
}

impl VNRecognizeTextRequest for id {
  unsafe fn init(self) -> id {
    msg_send![self, init]
  }

  unsafe fn results(self) -> id /* NSArray */ {
    msg_send![self, results]
  }
}

pub trait VNRecognizedTextObservation: Sized {
  unsafe fn topCandidates(self, maxCandidateCount: NSUInteger) -> id /* NSArray<VNRecognizedText *> */;
}

impl VNRecognizedTextObservation for id {
  unsafe fn topCandidates(self, maxCandidateCount: NSUInteger) -> id /* NSArray<VNRecognizedText *> */
  {
    msg_send![self, topCandidates: maxCandidateCount]
  }
}

pub trait VNRecognizedText: Sized {
  unsafe fn boundingBoxForRange(self, range: &NSRange, error: Option<*mut id /* NSError */>) -> id /* VNRectangleObservation */;
  unsafe fn string(self) -> id /* NSString */;
}

impl VNRecognizedText for id {
  unsafe fn boundingBoxForRange(self, range: &NSRange, error: Option<*mut id /* NSError */>) -> id /* VNRectangleObservation */
  {
    msg_send![
        self,
        boundingBoxForRange: *range
        error: error.unwrap_or(std::ptr::null_mut())
    ]
  }

  unsafe fn string(self) -> id /* NSString */ {
    msg_send![self, string]
  }
}
