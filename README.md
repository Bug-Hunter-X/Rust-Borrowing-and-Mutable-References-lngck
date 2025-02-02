# Rust Borrowing Example

This repository demonstrates a common error in Rust related to borrowing and mutable references.  The `bug.rs` file shows the error, while `bugSolution.rs` provides the corrected code.

## The Problem

Rust's ownership and borrowing system prevents data races and other concurrency issues. However, understanding how mutable and immutable references interact is crucial.  The example in `bug.rs` attempts to create two mutable references to the same value, leading to a compile-time error.

## The Solution

`bugSolution.rs` illustrates how to correctly use mutable and immutable references. It demonstrates how one mutable reference can exist at a time.