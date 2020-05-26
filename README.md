# sysrepo-rs
Sysrepo Rust binding.
Sysrepo is a YANG-based configuration and operational state data store for Unix/Linux applications.

## Requirement
Sysrepo-rs depends on [sysrepo][1] C library, which depdeds on libyang and some other libraries.  Please refer the following project repo to install appropriate dependencies to your system.

https://github.com/sysrepo/sysrepo

## Note
This crate is bare minimum auto generated bindings, so most of the function calls to sysrepo are unsafe.  There are some examples available in this repo.

[1]: http://www.sysrepo.org/
