# ruby-sys

Low-level bindings to Ruby, for Rust.

[![Build Status](https://travis-ci.org/steveklabnik/ruby-sys.svg?branch=master)](https://travis-ci.org/steveklabnik/ruby-sys)

## Building with non-default Rubies

By default, the bindings use the `ruby` in the system path to determine the build parameters. If a
non-system Ruby needs to be used, use the `RUBY` environment variable to specify the absolute path
to the Ruby executable.
