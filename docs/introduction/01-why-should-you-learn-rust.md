# Why should you learn Rust?

At Info Support we already work with quite a few programming language such as Java, C#, Typescript, Javascript, Python.
Why would you learn yet another language? Let's explore what makes Rust interesting and convince you to try it for
at least one afternoon.

## What is Rust?

Rust is a modern language with a focus on performance, and safety. It has different take on memory management that makes
it unique when compared to the other languages that we use at Info Support. It has no garbage collector that's in your
way.

Rust has a C-like syntax just like Java, C#, and Typescript/Javascript. It looks familiar to people coming from any
of the other languages. Which means that it is reasonably easy to start with.

Finally, the Rust language is interesting because it doesn't have the usual object oriented structures. It does have
structs (class-like things), and traits (interface-like things). But they are quite different from the object oriented
approach that many of the other languages stick to.

## Where can you use Rust?

Rust is very fast because it lacks a garbage collector and doesn't use a virtual machine approach. The performance of
Rust makes it useful for applications such as the Windows or Linux kernel because these things need to be fast.
Real-time embedded systems is another space where Rust is useful, because you can work within very limited resource
constraints with Rust. As you can imagine, people are using Rust for data engineering and AI as well for its performance.

You can also use Rust to enhance the experience with other languages. Many of the development tools for Python are
built with Rust because it's much faster than Python itself. It's surprising to see how many people are building
developer tools for other languages in Rust. 

We also found that Rust is great for green computing because its zero-cost abstraction approach reduces the amount of 
overhead when running the software. Rust runs well on low-powered environments, too, for the same reason.

Finally, we can recommend Rust for mission critical applications for its safety features. Rust is built around the idea
that the compiler can check how you manage memory. Its tight memory management regime prevents many of the usual memory 
bugs. It's also good to know that thread safety is implemented in the compiler. Finally, Rust has a great
type system that's very robust, so you can be sure the app works when it compiles.

## How productive is Rust?

You've probably read about Rust somewhere. Many people who haven't used Rust believe that Rust is hard to work with
and not very productive.

It is true that Rust requires effort to learn because of how different it is from other languages. However, with a few
good tutorials you can be on your way quite quickly. We recommend skipping over the more advanced parts to speed up
your journey. You're not going to need them in most cases.

Once you get the hang of the syntax, you'll find that Rust is quite productive because of the many great libraries 
you can use. In the rest of the tutorial, we'll point out a few interesting ones that we think are useful for many
applications.

## What's included in the box?

Rust, like many other languages, isn't just a language. It includes tools that will help you write effective code.
Let's go over the stuff that's included in the box.

### Package management

When you install Rust, you also get [Cargo](https://github.com/rust-lang/cargo) installed. With this tool, you can
manage Rust projects. Cargo allows you to create new projects, build projects for various operating systems, and run
tests. It also includes functions to manage project dependencies. You'll see us use Cargo a lot during the tutorial.

### Testing support

Rust has built-in support for running unit tests and system tests. Unit tests are included in individual modules you
build. System tests are included at the package level. We'll explore building unit tests as part of the tutorial.

### Linting support

The makers of Rust are quite keen on code quality. Rust comes with [Clippy](https://github.com/rust-lang/rust-clippy) to
help you format and verify code structure on your local machine or as part of the CI pipeline. The defaults for this tool
are great, but you can configure things if you need to make an exception on the defaults.

### Documentation

Rust includes syntax to write inline documentation. The inline documentation syntax in Rust allows you to write 
method-level documentation as well as overall documentation. The documentation is generated using 
[rustdoc](https://doc.rust-lang.org/rustdoc/index.html). You'll find that most developers publish their documentation
on [docs.rs](https://docs.rs) straight from their CI pipeline.

## Summary

Overall, you could say that Rust is a pretty complete language with some interesting things to offer. However, we
strongly believe that you should try it to get a good feel for it.

In [the next section](./02-learning-the-basics-of-rust.md) we'll cover the basics of the language in greater depth
and help you get started building your first hello world application.
