# explain-rs
A library which finds language features in rust code and provides resources on them.

### Give your co-workers a quickstart into your rust code's syntax.
Introducing Rust in existing teams can be difficult. Some members might work primarily in other languages and therefore can't devote much time to learn rust in all its facets.
Code in an unfamiliar syntax looks like gibberish at first, whilst the underlying concepts might in fact be similar with a known language.

That's where tooling comes into play: Paste your code / let it run on your source and it'll link to e.g. the rust book page for `destructuring` patterns for further reading.
Ideally, this will make it possible for rust beginners to dive into existing, expressive code bases, while learning rust on the fly.

### Intended Workflow
 1. Paste your code
 2. Move your cursor to a thing you don't understand.
 3. Read the context sensitive explanation
 4. Profit
 
### Features
 - [x] Context sensitive explanation
     - [x] Syntax
     - [ ] Controlflow
     - [ ] Lifetimes (Venn Diagram style?)

### Please provide rust source snippets with (exotic) syntax

Please [open issues](https://github.com/MSleepyPanda/explain-rs/issues/new) with syntax you'll find hard to grasp. Unless already covered (if in doubt, open issue anyway), these will be integrated into our syntax studies.
This will give us a good feeling on where to focus in the beginning.

### Goals
Eventually, we want to support the rust syntax completely. Things i'd like to explore at that point would be pattern analysis and type level integration via RLS.

### Status
Pre alpha. Don't expect it to work on your code.

### License

explain-rs is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
