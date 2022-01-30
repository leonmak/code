#![allow(dead_code)] // <1>
use std::fmt; // <2>
use std::{thread, time};

use chrono::prelude::*;

#[derive(Debug)]
struct CubeSat<'a> {
  id: u64,
  name: String,
  mailbox: Mailbox<'a>,
}

#[derive(Debug)]
struct Mailbox<'a> {
  messages: Vec<&'a Message>,
}

#[derive(Debug)]
struct Message {
  sender: u64,
  content: String,
}

// struct GroundStation<'a> {
//   name: &'a str,
// }

struct GroundStation<'a> {
  gsid: u64,
  name: &'a str,
  mailbox: Mailbox<'a>,
}

/// Actor contains a mailbox, and sends messages to each other
trait Actor<'a> {
  fn recv(self: &'a mut Self) -> &mut Self;
  fn send(self: &'a Self, to: &mut CubeSat<'a>, msg: &'a Message);
}

struct PostAction<'a> {
  msg: &'a Message,
  to: &'a mut CubeSat<'a>,
}

struct ReceiveAction<'a> {
  rcv: &'a mut CubeSat<'a>,
  time: DateTime<Local>,
}

trait Formatter {
  fn format(self: &Self);
}

impl fmt::Display for Message {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let m = &mut String::new();
    // NOTE: define mutable reference from outside calling function
    // cannot return stuff in function
    self.shorten(m);
    write!(formatter, "Message( sender:{}, msg:'{}' )", self.sender, m)
  }
}

const GS_MAX_LEN: usize = 6;
// impl<'a> GroundStation<'a> {
impl<'a> Actor<'a> for GroundStation<'a> {
  fn send(self: &Self, to: &mut CubeSat<'a>, msg: &'a Message) {
    // let post: PostAction = PostAction { to: to, msg };
    // post.execute();
    to.mailbox.messages.push(msg);
    let local: DateTime<Local> = Local::now();
    println!("gs({}) sent {} to csat{} on {}", self.gsid, msg, to.id, local);
  }

  fn recv(&mut self) -> &mut Self {
    // todo: handle dead sats
    self
  }
}

impl<'a> Actor<'a> for CubeSat<'a> {
  fn send(self: &Self, _to: &mut CubeSat, _msg: &Message) {
    // todo: send to other sat / report to base if fail
  }

  fn recv(&'a mut self) -> &mut Self {
    let local: DateTime<Local> = Local::now();
    let msg: Option<&Message> = self.mailbox.messages.pop();
    match msg {
      Some(ms) => {
        println!("{}, received by csat{}, on {}", ms, self.id, local);
      }
      None => {
        println!("No more messages");
      }
    }
    self
  }
}

// run command containing needed info
// // todo: make it concurrent
// trait Executor {
//   fn execute(self: &Self);
// }

// impl Executor for PostAction<'_> {
//   fn execute(&self) {
//     self.to.mailbox().messages.push(self.msg);
//     self.describe(self.msg.msg.shorten());
//   }
// }

// impl Executor for ReceiveAction<'_> {
//   fn execute(&self) {
//     let msg: Option<Message> = self.rcv.mailbox.messages.pop();
//     self.describe(msg.shorten());
//   }
// }

// used for shorter descriptions in describe()
trait Shortener {
  fn shorten(&self, m: &mut String);
}

impl Shortener for Message {
  fn shorten(&self, m: &mut String) {
    let mut short_msg: String;
    if self.content.len() > GS_MAX_LEN {
      short_msg = String::from(&self.content[..GS_MAX_LEN]);
      short_msg.push_str("..");
    } else {
      short_msg = self.content.to_string();
    };
    m.clone_from(&short_msg);
  }
}

// describe()
// trait ActionDescriptor {
//   fn describe(&self, msg: &Message);
// }

// impl ActionDescriptor for PostAction<'_> {
//   fn describe(&self, msg: &Message) {
//     println!("{} sent {} to {} mailbox", self.from, msg, self.to);
//   }
// }

// impl ActionDescriptor for ReceiveAction<'_> {
//   fn describe(&self, msg: &Message) {
//     println!("{} received {} from {}", self.to, msg, self.from);
//   }
// }

fn main() {
  let base = GroundStation {
    gsid: 6,
    name: "G6",
    mailbox: Mailbox { messages: vec![] },
  };
  let mut sat_a = CubeSat {
    id: 0,
    name: String::from("sat_a"),
    mailbox: Mailbox { messages: vec![] },
  };
  println!("\nStart Run\nt0: {:?}\n", sat_a);

  // base send to sat_a mail
  // PostAction {}
  let msg1 = Message {
    sender: base.gsid,
    content: String::from("hello there sat_a!"),
  };
  
  println!("gs6 Sending msg1..");
  base.send(&mut sat_a, &msg1); // <1>
  println!("t1: {:?}\n", sat_a);
  let time_s = 1;  
  thread::sleep(time::Duration::from_secs(time_s));

  // todo: timeout and run recv in separate threads
  println!("csat6 sleeping {:?}s..", time_s);
  thread::sleep(time::Duration::from_secs(time_s));

  //sat_a mail recv to sat_a
  // NOTE: return ownership to calling function
  let sat_a = sat_a.recv();
  println!("t2: {:?}", sat_a);
  // println!("msg: {:?}", msg);
}
