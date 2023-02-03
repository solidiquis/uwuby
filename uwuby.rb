require 'benchmark'
require 'ffi'
require "test/unit"

include Test::Unit::Assertions 

module Rust
  extend FFI::Library
  ffi_lib './target/release/libruby_rs.dylib'
  attach_function :fib, [:int], :int
end

assert_equal(Rust.fib(40), 102334155)
