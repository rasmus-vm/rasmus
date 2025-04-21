# Tasks

- [ ] errors hierarchy
- [ ] dependencies
  - [ ] mechanism to provide modules with names
    - [ ] investigate existing standards
    - [ ] if not found define own stadard
    - [ ] impelement selected standard: imports resolution via module registry, hierarchical module instantiation
  - [x] mechanism to select a module by name to run
- [ ] host modules
  - [ ] mechanism to run host code similarly to running regular non-host functions
  - [ ] implement `console` module
  - [ ] implement testing module to run official spec tests https://github.com/WebAssembly/testsuite
- [ ] extend `Trap` to provide exact information about the reason
- [ ] refine validation errors to make them more informative
- [ ] improve output
- [ ] separate CLI from the core crate, publish core crate to crates.io