// Copyright (c) 2014 Marshall A. Greenblatt. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the name Chromium Embedded
// Framework nor the names of its contributors may be used to endorse
// or promote products derived from this software without specific prior
// written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// ---------------------------------------------------------------------------
//
// This file was generated by the CEF translator tool and should not be edited
// by hand. See the translator.README.txt file in the tools directory for
// more information.
//

#![allow(non_snake_case, unused_imports)]

use eutil;
use interfaces;
use types;
use wrappers::CefWrap;

use libc;
use std::collections::HashMap;
use std::ptr;

//
// Implement this structure to handle events related to keyboard input. The
// functions of this structure will be called on the UI thread.
//
#[repr(C)]
pub struct _cef_keyboard_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  // Called before a keyboard event is sent to the renderer. |event| contains
  // information about the keyboard event. |os_event| is the operating system
  // event message, if any. Return true (1) if the event was handled or false
  // (0) otherwise. If the event will be handled in on_key_event() as a keyboard
  // shortcut set |is_keyboard_shortcut| to true (1) and return false (0).
  pub on_pre_key_event: Option<extern "C" fn(this: *mut cef_keyboard_handler_t,
      browser: *mut interfaces::cef_browser_t,
      event: *const interfaces::cef_key_event_t,
      os_event: types::cef_event_handle_t,
      is_keyboard_shortcut: *mut libc::c_int) -> libc::c_int>,

  //
  // Called after the renderer and JavaScript in the page has had a chance to
  // handle the event. |event| contains information about the keyboard event.
  // |os_event| is the operating system event message, if any. Return true (1)
  // if the keyboard event was handled or false (0) otherwise.
  //
  pub on_key_event: Option<extern "C" fn(this: *mut cef_keyboard_handler_t,
      browser: *mut interfaces::cef_browser_t,
      event: *const interfaces::cef_key_event_t,
      os_event: types::cef_event_handle_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_keyboard_handler_t = _cef_keyboard_handler_t;


//
// Implement this structure to handle events related to keyboard input. The
// functions of this structure will be called on the UI thread.
//
pub struct CefKeyboardHandler {
  c_object: *mut cef_keyboard_handler_t,
}

impl Clone for CefKeyboardHandler {
  fn clone(&self) -> CefKeyboardHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefKeyboardHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefKeyboardHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefKeyboardHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_keyboard_handler_t) -> CefKeyboardHandler {
    CefKeyboardHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_keyboard_handler_t) -> CefKeyboardHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefKeyboardHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_keyboard_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_keyboard_handler_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  // Called before a keyboard event is sent to the renderer. |event| contains
  // information about the keyboard event. |os_event| is the operating system
  // event message, if any. Return true (1) if the event was handled or false
  // (0) otherwise. If the event will be handled in on_key_event() as a keyboard
  // shortcut set |is_keyboard_shortcut| to true (1) and return false (0).
  pub fn on_pre_key_event(&self, browser: interfaces::CefBrowser,
      event: &interfaces::CefKeyEvent, os_event: types::cef_event_handle_t,
      is_keyboard_shortcut: &mut libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_pre_key_event.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(event),
          CefWrap::to_c(os_event),
          CefWrap::to_c(is_keyboard_shortcut)))
    }
  }

  //
  // Called after the renderer and JavaScript in the page has had a chance to
  // handle the event. |event| contains information about the keyboard event.
  // |os_event| is the operating system event message, if any. Return true (1)
  // if the keyboard event was handled or false (0) otherwise.
  //
  pub fn on_key_event(&self, browser: interfaces::CefBrowser,
      event: &interfaces::CefKeyEvent,
      os_event: types::cef_event_handle_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_key_event.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(event),
          CefWrap::to_c(os_event)))
    }
  }
} 

impl CefWrap<*mut cef_keyboard_handler_t> for CefKeyboardHandler {
  fn to_c(rust_object: CefKeyboardHandler) -> *mut cef_keyboard_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_keyboard_handler_t) -> CefKeyboardHandler {
    CefKeyboardHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_keyboard_handler_t> for Option<CefKeyboardHandler> {
  fn to_c(rust_object: Option<CefKeyboardHandler>) -> *mut cef_keyboard_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_keyboard_handler_t) -> Option<CefKeyboardHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefKeyboardHandler::from_c_object_addref(c_object))
    }
  }
}
