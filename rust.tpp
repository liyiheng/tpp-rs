--title Rust
--author WaySLOG
--date today

--newpage intro
--heading Introduction

---
  * memory safe
  * native thread
  * without GC pause
  * trait base programming 
  * zero cost abstraction
  * closure
  * no data racing
  * can use unsafe
  * easy ffi call
  * pattern match
  * so nice a compiler

---
but, people always said it's difficult to learn, really ?

--newpage typical-rust
--beginoutput
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}
--endoutput

--newpage ownership system
--heading OwnerShip && LifeTime

Rust's type system base:

---

--huge ownership


---
Think that we have a lock in each variable.
---

--boldon
In compile Time
--boldoff

---
let us think about typical case: iterator and push_back

--newpage iterator-in-cpp
--heading Iterator Invalid(1)

--center visit url: http://cpp.sh/67zt7

--beginoutput

vector<int> a = {1};

a.push_back(2);
    
auto it = a.begin();

for (int i = 0; i < 10000; ++i) {
    a.push_back(i);
}

cout << *it << endl;

--endoutput

--newpage desc-iterator-ini-cpp
--heading Iterator Invalid(2)

vector.data()'s memory address may be changed (point: may be !!!),

but iterator still hold the original address,

so the iterator was invalid right now.

how to deal with that ...

---
in Rust ?

--newpage Rust's OwnerShip
--heading OwnerShip

principle:

  * each variable has only one owner, clone make a new variable
  * owner can borrow one mutable once only if there is no any other borrow
  * owner can borrow one or more immutable borrow only if there is no other mutable borrow

---
all just like a ReadWriteLock .

--newpage ownership
--heading OwnerShip example

I have a ownership of the Book, I can modify and give it to other. 
--beginoutput
struct Book {
    name: String,
}

fn give(books: Vec<Book>) { ... }

fn foo() {
    // mut means that can be modified
    let mut books: Vec<Book> = Vec::new();
    books.push(Book { name: "Time".to_string() });
    give(books);
    give(books); // use of moved value: `books`
}
--endoutput

--newpage borrow
--heading Borrow

I can borrow my book to anyone who want to read.

--beginoutput
fn read(book: &Book) { ... }
fn bar() {
    let book = Book { name: "time".to_string() };
    let brw1 = &book; // Ok
    let brw2 = &book; // Ok

    // No: cannot borrow immutable local variable `book` as mutable
    let mut mut_brw = &mut book;
}
--endoutput

--newpage lifetime
--heading LifeTime

I promise that you can read the book until die.

--beginoutput
fn bar() { // think a code block is a scope
    let book = Book { name: "time".to_string() };
    // brw1's lifetime start
    let brw1 = &book;
    // brw2's lifetime start
    let brw2 = &book; 
    // brw2 drop second
    // brw1 drop third
    // book drop at last
}
--endoutput

--newpage use lifetime in function
--heading Use LifeTime as a parameter

--beginoutput
1. fn choice<'a, 'b>(str1: &'a str, str2: &'a str)
2.                  -> &'??? str {...}
--endoutoput

what lifetime should the function return ?

--heading Use LifeTime as a parameter



--beginoutput

--endoutput

--newpage RAII-and-lifetime-1
--heading RAII and lifetime(1)

first, we consider about RAII : Resource Acquisition Is Initialization.

---
destroy resource by stack sequence.

--beginoutput
// in rust
fn open_file() {
   let file = File::open("./demo.txt").except("can't open file");
   // auto drop and call File::close
}

// in golang
func OpenFile() {
    fp, err := os.Open("./demo.txt");
    if err != nil {
       glog.Error("can't open file")
       panic("")
    }
    defer func() {
        if err := fi.Close(); err != nil {
            panic(err)
        }
    }()
}

// _(´ཀ`」 ∠)_
--endoutput

--newpage RAII-and-lifetime-2
--heading RAII and lifetime(2)

ownered object lifetime is only in block otherwise was returned (moved) .

but consider about borrow lifetime.

--beginoutput

let value = 1;
let brrw1 = &value;
let brrw2 = &value;
--endoutput

which one was destroy first ?

---
in stack order of course.

brrw1's lifetime is larger than brrw2.

--newpage RAII-and-lifetime-3
--heading RAII-and-lifetime(3)

my practice:

    don't write any lifetime when you don't need it.

--newpage concurrency
--heading concurrency

shared mutable state is the root of all evil.

we need lock to prevent us from unexpect access, but ...

lock for what ? process ?
---

no !

we always need to lock what could change in concurrency, data, called in Rust.

--beginoutput
// a locker demo
let mut locker = Mutex::new(vec![]); 
let mut value = locker.lock().unwrap();
value.push(1);
// deal to RAII, unlock was auto called
--endoutput

that's enough ? no ! in mulity threads.

--newpage sync-send
--heading Sync/Send

first, look the trait Sync and Send

    A type is Send if it is safe to send it to another thread.
    A type is Sync if it is safe to share between threads (&T is Send).

there is more addition:
    A type T is Sync if &T is Send

so what's wrong in previous page ?

Mutex is always moved by FnOnce signature.

--beginoutput

// concurrency access 
let locker = Arc::new(Mutex::new(vec![]));

--endoutput

in simple description:
   * Sync provide access in immutable way
   * Mutex provide mutable access by inter mutable 

--newpage out of topic: lock coast
--heading [OT] - really coast in lock

use never faild syscall: getpid , interaptor into kernel about: 50ns .

test url: https://gist.github.com/wayslog/b4ae56b24dcf72d12052b53103c4d889

--beginoutput
test test_lock_in_1_thread  ... bench:      57,231 ns/iter (+/- 2,738)
test test_lock_in_2_thread  ... bench:      88,636 ns/iter (+/- 4,005)
test test_lock_in_4_thread  ... bench:     150,765 ns/iter (+/- 6,826)
test test_lock_in_8_thread  ... bench:     275,251 ns/iter (+/- 13,892)
test test_lock_in_16_thread ... bench:     523,795 ns/iter (+/- 24,552)
test test_lock_in_32_thread ... bench:   1,017,843 ns/iter (+/- 35,990)
--endoutput

more time was spend on context switch.

in fact that's not too bad because mutex's implemention is futex .
which mixed spinlock and mutex.

why the grap is reached best performance when at 8 thread ?

because i have 8 core ...

--newpage other-deal
--heading script language how to deal this question

maybe we can set lock for each variable...
---
but, it's actually slower than one lock.

---
Python: GIL(Global Interpreter Lock)

---
maybe a good solution, but ... how to use multiple cpu core ?

that's a undocumented feature, not a bug in design !


--newpage communication between threads
--heading communication between threads

Rust privode a mulity producer single consumer channel, communicate is easy:

--beginoutput
use std::sync::mpsc::channel;
use std::thread;

let (tx, rx) = channel();

// Spawn off an expensive computation
thread::spawn(move|| {
    tx.send(expensive_computation()).unwrap();
});

// Do some useful work for awhile
// Let's see what that answer was
println!("{:?}", rx.recv().unwrap());

--endoutput

--newpage little about closure
--heading little about closure

Closure is just a anonymous struct in Rust.

but impl Fn FnMut FnOnce.

--beginoutput

let name = "elton".to_string();
// x impl what ???
let x = || {println!("name is {}", &name); };

// the same as above
// but you could never write this...
// notice extern "rust-call" marker in Fn trait
pub struct Closure1<'a> {
    _field1: &'a String,
}

impl<'a> Fn for Closure1<'a> {
    fn call(&self, args: Args) -> Self::Output {
       let x = || {println!("name is {}", &self.field1); };
    }
}
--endoutput
---

Order: Fn > FnMut > FnOnce

if you want to use FnOnce, key world move is almost always use.


--newpage little practice
--heading Practice: newbee & lin-rs

url: https://crates.io/crates/newbee

--newpage macro

macro: remove indent, repeat work

example:
--beginoutput
macro_rules! more {
    ($e: expr) => {
        if $e { return Err(Error::More); }
    }
}

macro_rules! choice {
        ($e: expr) => {
            match $e {
                Ok(lp) => return Ok(lp),
                Err(Error::Other) => {}
                Err(err) => return Err(err),
            };}
    }
--endoutput

--newpage macro-example-1
--heading example usage:

--beginoutput
impl FromBuf for ZLESpData {
    fn from_buf(src: &[u8]) -> Result<ZLESpData> {
        more!(src.len() == 0);
        choice!(ZLESpData::to_str(src));
        choice!(ZLESpData::to_usual_int(src));
        choice!(ZLESpData::to_special_int(src));
        Err(Error::Faild("not regular ZipListSpecialFlag"))
    }
}
--endoutput

--newpage macro-example
--heading Macro Example

url: https://github.com/wayslog/lin-rs/blob/master/src/memset.rs#L28

--beginoutput

pub struct TimeSet {
    inmap: HashMap<Index, Vec<f64>>,
}

pub trait Truncate {
    fn truncate(&mut self) -> Self;
}

macro_rules! impl_truncate {
    ($set:ty) => {
        impl Truncate for $set {
            fn truncate(&mut self) -> Self {
                let mut inmap = HashMap::new();
                mem::swap(&mut self.inmap, &mut inmap);
                Self {
                    inmap:inmap,
                }
            }
        }
    }
}

impl_truncate!(TimeSet);
impl_truncate!(CountSet);
impl_truncate!(GaugeSet);
--endoutput

--newpage trait
--Heading Two Trait

--beginoutput
/// define parse process
pub trait FromBuf
    where Self: Sized
{
    fn from_buf(src: &[u8]) -> Result<Self>;
}
--endoutput

--beginoutput
/// define pared unit length
pub trait Shift {
    // inline is a attribute set inline
    #[inline]
    fn shift(&self) -> usize;
}
--endoutput

implement a trait for parsed unit: ——

like RedisString !

--newpage redis string
--heading RedisString

--beginoutput
#[derive(Debug, Clone)]
pub enum RedisString {
    LengthPrefix { len: Length, data: Vec<u8> },
    StrInt(StrInt),
    LZF(LZFString),
}
impl Shift for RedisString {
    fn shift(&self) -> usize {
        match self {
            &RedisString::LengthPrefix { ref len, ref data } => len.shift() + data.len(),
            &RedisString::StrInt(ref ival) => ival.shift(),
            &RedisString::LZF(ref lzf) => lzf.shift(),
        }
    }
}
impl FromBuf for RedisString {
    fn from_buf(src: &[u8]) -> Result<RedisString> {
        choice!(RedisString::length_prefix(src));
        choice!(RedisString::str_int(src));
        choice!(RedisString::lzf(src));
        Err(Error::Faild("can't parse buffer as RedisString"))
    }
}
--endoutput

--newpage enhance std
--heading Enhance Std 

can we implement a trait for a primitives ?

of course we can.

example:
---
--beginoutput
impl Shift for u32 {
    #[inline]
    fn shift(&self) -> usize {
        4
    }
}
impl FromBuf for u32 {
    fn from_buf(src: &[u8]) -> Result<u32> {
        more!(src.len() < 4);
        Ok(buf_to_u32(src))
    }
}
--endoutput

--newpage Q & A
--heading Q & A

--newpage thanks
--heading Thanks all