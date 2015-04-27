/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// Liberally derived from the [Firefox JS implementation](http://mxr.mozilla.org/mozilla-central/source/toolkit/devtools/server/actors/webconsole.js).
/// Mediates interaction between the remote web console and equivalent functionality (object
/// inspection, JS evaluation, autocompletion) in Servo.

extern crate hyper;
extern crate url;

use actor::{Actor, ActorRegistry};
use protocol::JsonPacketStream;

use devtools_traits::DevtoolScriptControlMsg;
use msg::constellation_msg::PipelineId;
use devtools_traits::{DevtoolsControlMsg, NetworkEvent};

use collections::BTreeMap;
use core::cell::RefCell;
use std::fmt;
use rustc_serialize::json::{self, Json, ToJson};
use std::net::TcpStream;
use std::num::Float;
use std::sync::mpsc::{channel, Sender};
use std::borrow::IntoCow;

use url::Url;
use hyper::header::Headers;
use hyper::http::RawStatus;
use hyper::method::Method;

struct HttpRequest {
    url: String,
    method: Method,
    headers: Headers,
    body: Option<Vec<u8>>,
}

struct HttpResponse {
    headers: Option<Headers>,
    status: Option<RawStatus>,
    body: Option<Vec<u8>>
}

#[derive(RustcEncodable)]
struct GetRequestHeadersReply {
    from: String,
    headers: String,
    headerSize: u8,
    rawHeaders: String
}

#[derive(RustcEncodable)]
pub struct EventActor {
    pub actor: String,
    pub url: String,
    pub method: String,
    pub startedDateTime: String,
    pub isXHR: String,
    pub private: String
}

#[derive(RustcEncodable)]
pub struct ResponseStartMsg {
    pub httpVersion: String,
    pub remoteAddress: String,
    pub remotePort: u8,
    pub status: String,
    pub statusText: String,
    pub headersSize: u8,
    pub discardResponseBody: bool,
}

pub struct NetworkEventActor {
    pub name: String,
    request: HttpRequest,
    response: HttpResponse,
}

impl Actor for NetworkEventActor {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle_message(&self,
                      _registry: &ActorRegistry,
                      msg_type: &str,
                      msg: &json::Object,
                      stream: &mut TcpStream) -> Result<bool, ()> {
        Ok(match msg_type {

            "getRequestHeaders" => {
                println!("getRequestHeaders");
                let msg = GetRequestHeadersReply {
                    from: self.name(),
                    headers: "headers".to_string(),
                    headerSize: 10,
                    rawHeaders: "Raw headers".to_string(),                    
                };
                stream.write_json_packet(&msg);
                true
            }

            "getRequestCookies" => {
                println!("getRequestCookies");
                true
            }
            
            "getRequestPostData" => {
                println!("getRequestPostData");
                true
            }

            "getResponseHeaders" => {
                println!("getResponseHeaders");
                true
            }

            "getResponseCookies" => {
                println!("getResponseCookies");
                true
            }

            "getResponseContent" => {
                println!("getResponseContent");
                true
            }

            _ => false
        })
    }
}

impl NetworkEventActor {
    pub fn new(name: String) -> NetworkEventActor {
        NetworkEventActor {
            name: name,
            request: HttpRequest {
                url: String::new(),
                method: Method::Get,
                headers: Headers::new(),
                body: None
            },
            response: HttpResponse {
                headers: None,
                status: None,
                body: None,
            }
        } 
    }

    pub fn addEvent(&mut self, network_event: NetworkEvent) {
        match network_event {
            NetworkEvent::HttpRequest(url, method, headers, body) => {
                self.request.url = url.serialize();
                self.request.method = method.clone();
                self.request.headers = headers.clone();
                self.request.body = body;
            }    
            NetworkEvent::HttpResponse(headers, status, body) => {
                self.response.headers = headers.clone();
                self.response.status = status.clone();
                self.response.body = body.clone();
            }
        }
    }

    pub fn get_event_actor(&self) -> EventActor {
        EventActor {
            actor: self.name(),
            url: self.request.url.clone(),
            method: format!("{}", self.request.method),
            startedDateTime: "2015-04-22T20:47:08.545Z".to_string(),
            isXHR: "false".to_string(),
            private: "false".to_string(),
        }
    }

    pub fn get_response_start(&self) -> ResponseStartMsg {
        ResponseStartMsg {
            httpVersion: "HTTP/1.1".to_string(),
            remoteAddress: "63.245.217.43".to_string(),
            remotePort: 443,
            status: "200".to_string(),
            statusText: "OK".to_string(),
            headersSize: 337,
            discardResponseBody: true
        }
    }
}
