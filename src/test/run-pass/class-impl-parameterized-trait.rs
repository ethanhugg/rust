// xfail-test
// xfail-fast

extern mod std;
use std::map::*;

class cat : map<int, bool> {
  priv {
    // Yes, you can have negative meows
    let mut meows : int;
    fn meow() {
      self.meows += 1;
      error!("Meow %d", self.meows);
      if self.meows % 5 == 0 {
          self.how_hungry += 1;
      }
    }
  }

  let mut how_hungry : int;
  let name : str;

  new(in_x : int, in_y : int, in_name: str)
    { self.meows = in_x; self.how_hungry = in_y; self.name = in_name; }

  fn speak() { self.meow(); }

  fn eat() -> bool {
    if self.how_hungry > 0 {
        error!("OM NOM NOM");
        self.how_hungry -= 2;
        return true;
    }
    else {
        error!("Not hungry!");
        return false;
    }
  }

  fn size() -> uint { self.meows as uint }
  fn insert(+k: int, +v: bool) -> bool { 
    if v { self.meows += k; } else { self.meows -= k; };
    true
  }
  fn contains_key(&&k: int) -> bool { k <= self.meows }
  fn get(&&k:int) -> bool { k <= self.meows }
  fn [](&&k:int) -> bool { k <= self.meows }
  fn find(&&k:int) -> option<bool> { some(self.get(k)) }
  fn remove(&&k:int) -> option<bool> { self.meows -= k; some(true) }
  fn each(f: fn(&&int, &&bool) -> bool) {
    let mut n = int::abs(self.meows);
    while n > 0 {
        if !f(n, true) { break; }
        n -= 1;
    }
  }
  fn each_key(&&f: fn(&&int) -> bool) {
    for self.each |k, _v| { if !f(k) { break; } again;};
  }
  fn each_value(&&f: fn(&&bool) -> bool) {
    for self.each |_k, v| { if !f(v) { break; } again;};
  }
  fn clear() { }
}

fn main() {
  let nyan : cat = cat(0, 2, "nyan");
  for uint::range(1u, 5u) |_i| { nyan.speak(); }
  // cat returns true if uint input is greater than
  // the number of meows so far
  assert(nyan.get(1));
  assert(!nyan.get(10));
}
