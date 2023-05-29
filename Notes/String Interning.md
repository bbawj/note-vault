---
title: "String Interning"
date: 2023-05-28
lastmod: 2023-05-28
---
# String Interning
When comparing 2 strings, the only way to ensure that they are both the same in value, is to walk the entire string and compare each byte. This is a time consuming process, especially when there are alot of string comparisons.

String interning is a process of deduplication. We create a collection of “interned” strings. Any string in that collection is guaranteed to be textually distinct from all others. When you intern a string, you look for a matching string in the collection. If found, you use that original one. Otherwise, the string you have is unique, so you add it to the collection.

By interning each string when it is created, we do upfront work to ensure duplicate strings point to the same memory address, reducing the total memory use.
## Rust solutions
Naive solution with 2 allocations.
```rust
#[derive(Default)]

pub struct Interner {
    map: HashMap<String, u32>,
    vec: Vec<String>,
}

impl Interner {
    pub fn intern(&mut self, name: &str) -> u32 {
        if let Some(&idx) = self.map.get(name) {
            return idx;
        }
        let idx = self.map.len() as u32;
        self.map.insert(name.to_owned(), idx);
        self.vec.push(name.to_owned());

        debug_assert!(self.lookup(idx) == name);
        debug_assert!(self.intern(name) == idx);

        idx
    }

    pub fn lookup(&self, idx: u32) -> &str {
        self.vec[idx as usize].as_str()
	}
}
```
### Optimised with 1 allocation
The trick is to add strings to buf in such a way that they are never moved, even if more strings are added on top. That way, we can just store &str in the HashMap. To achieve address stability, we use another trick from the typed_arena crate. If the buf is full (so that adding a new string would invalidate old pointers), we allocate a new buffer, twice as large, without coping the contents of the old one.
```rust
pub struct Interner {
    map: HashMap<&'static str, u32>,
    vec: Vec<&'static str>,
    buf: String,
    full: Vec<String>,
}

impl Interner {
    pub fn with_capacity(cap: usize) -> Self {
        let cap = cap.next_power_of_two();
        Self {
            map: HashMap::new(),
            vec: Vec::new(),
            buf: String::with_capacity(cap),
            full: Vec::new(),
        }
    }

    pub fn intern(&mut self, name: &str) -> u32 {
        if let Some(id) = self.map.get(name) {
            return *id;
        }

        let cap = self.buf.capacity();
        if cap < self.buf.len() + name.len() {
            let new_cap = (cap.max(name.len()) + 1).next_power_of_two();
            let new_buf = String::with_capacity(new_cap);
            let old_buf = std::mem::replace(&mut self.buf, new_buf);
            self.full.push(old_buf);
        }

        let start = self.buf.len();
        self.buf.push_str(name);
        let new_name = &self.buf[start..];
        let interned: &str;

        unsafe {
            interned = &*(new_name as *const str);
        }

        let id = self.map.len() as u32;
        self.map.insert(interned, id);
        self.vec.push(interned);

        id
    }

    pub fn lookup(&self, id: u32) -> &str {
        self.vec[id as usize]
    }
}

```