# Rust Knowledge Map

This document maps Rust programming concepts to the specific days and projects where they are taught and practiced in this curriculum. Use this to quickly find examples and explanations for specific topics.

## 🟢 1. Language Basics
| Concept | Day / Project | Description |
|:---|:---|:---|
| **Variables & Mutability** | [Day 02](Day01-15/02.VariablesAndTypes/README.md) | `let`, `mut`, shadowing, constants |
| **Data Types** | [Day 02](Day01-15/02.VariablesAndTypes/README.md) | Scalar types, Compound types (Tuples, Arrays) |
| **Functions** | [Day 03](Day01-15/03.FunctionsAndControlFlow/README.md) | Parameters, return values, statements vs expressions |
| **Control Flow** | [Day 03](Day01-15/03.FunctionsAndControlFlow/README.md) | `if`, `else`, `loop`, `while`, `for` |
| **Modules & Visibility** | [Day 09](Day01-15/09.Modules/README.md) | `mod`, `pub`, `use`, path resolution |

## 🔵 2. Ownership & Memory Safety
| Concept | Day / Project | Description |
|:---|:---|:---|
| **Ownership Rules** | [Day 04](Day01-15/04.Ownership/README.md) | Move semantics, Stack vs Heap, `Copy` trait |
| **References & Borrowing** | [Day 05](Day01-15/05.ReferencesAndBorrowing/README.md) | Immutable vs Mutable references, borrowing rules |
| **Slices** | [Day 06](Day01-15/06.Slices/README.md) | String slices (`&str`) and array slices |
| **Lifetimes** | [Day 14](Day01-15/14.Lifetimes/README.md) | Lifetime annotations, elision rules, `'static` |
| **Smart Pointers (Box)** | [Day 19](Day16-30/19.SmartPointers_Box/README.md) | Heap allocation with `Box<T>` |
| **RefCell & Interior Mutability** | [Day 20](Day16-30/20.SmartPointers_RcRefCell/README.md) | Runtime borrowing checks |
| **Reference Cycles** | [Day 21](Day16-30/21.ReferenceCycles/README.md) | `Weak<T>`, preventing leaks |

## 🟣 3. Type System & Abstraction
| Concept | Day / Project | Description |
|:---|:---|:---|
| **Structs** | [Day 07](Day01-15/07.Structs/README.md) | Defining and instantiating structs, methods |
| **Enums & Pattern Matching** | [Day 08](Day01-15/08.Enums/README.md) | `enum`, `match`, `Option<T>`, `if let` |
| **Generics** | [Day 12](Day01-15/12.Generics/README.md) | Generic functions, structs, and enums |
| **Traits** | [Day 13](Day01-15/13.Traits/README.md) | Defining shared behavior, trait bounds |
| **Trait Objects** | [Day 26](Day16-30/26.OOP_TraitObjects/README.md) | Dynamic dispatch, `dyn Trait` |
| **Advanced Traits** | [Day 28](Day16-30/28.AdvancedTraits/README.md) | Associated types, default parameters, supertraits |
| **Advanced Types** | [Day 29](Day16-30/29.AdvancedTypes/README.md) | Newtype pattern, type aliases, never type `!` |

## 🟠 4. Error Handling
| Concept | Day / Project | Description |
|:---|:---|:---|
| **Result & Option** | [Day 11](Day01-15/11.ErrorHandling/README.md) | Recoverable vs Unrecoverable errors |
| **panic!** | [Day 11](Day01-15/11.ErrorHandling/README.md) | Unwinding, aborting |
| **Propagation** | [Day 11](Day01-15/11.ErrorHandling/README.md) | The `?` operator |
| **Custom Errors** | [Day 43](Day31-45/43.ErrorHandling/README.md) | `thiserror`, `anyhow` crates |

## 🟡 5. Functional Features
| Concept | Day / Project | Description |
|:---|:---|:---|
| **Closures** | [Day 16](Day16-30/16.Closures/README.md) | Anonymous functions, capturing environment (`Fn`, `FnMut`, `FnOnce`) |
| **Iterators** | [Day 17](Day16-30/17.Iterators/README.md) | `Iterator` trait, adaptors (`map`, `filter`, `fold`) |

## 🔴 6. Concurrency & Async
| Concept | Day / Project | Description |
|:---|:---|:---|
| **Threads** | [Day 22](Day16-30/22.Threads/README.md) | `std::thread::spawn`, `join` |
| **Message Passing** | [Day 23](Day16-30/23.MessagePassing/README.md) | Channels (`mpsc`) |
| **Shared State** | [Day 24](Day16-30/24.SharedState/README.md) | `Mutex`, `Arc` |
| **Send & Sync** | [Day 25](Day16-30/25.SendSync/README.md) | Marker traits for concurrency safety |
| **Async/Await** | [Day 35](Day31-45/35.AsyncAwait/README.md) | Future trait, async syntax |
| **Tokio Runtime** | [Day 39](Day31-45/39.TokioDeepDive/README.md) | Tasks, I/O drivers, green threads |
| **Concurrency Patterns** | [Day 51](Day46-60/51.Concurrency/README.md) | Locks in async context, `RwLock` |

## 🟤 7. Web Development (Axum)
| Concept | Day / Project | Description |
|:---|:---|:---|
| **Web Server Basics** | [Day 40](Day31-45/40.AsyncWebServer/README.md) | Basic HTTP handling |
| **Routing & Handlers** | [Day 63](Day61-80/63.RoutingHandlers/README.md) | Path parameters, Query params, JSON body |
| **Middleware** | [Day 64](Day61-80/64.Middleware/README.md) | `tower-http`, custom layers |
| **State Management** | [Day 65](Day61-80/65.StateManagement/README.md) | `Extension`, `State` extractor |
| **Database (SQLx)** | [Day 41](Day31-45/41.Database_SQLx/README.md) | Async SQL, connection pools, migrations |
| **Authentication** | [Day 69](Day61-80/69.JWT/README.md) | JWT, Password hashing (`argon2`) |
| **OpenAPI** | [Day 74](Day61-80/74.OpenAPI/README.md) | `utoipa` for Swagger docs |

## ⚫ 8. System Programming & Low Level
| Concept | Day / Project | Description |
|:---|:---|:---|
| **File I/O** | [Day 31](Day31-45/31.FileIO/README.md) | Reading/Writing files, buffering |
| **Unsafe Rust** | [Day 36](Day31-45/36.UnsafeRust/README.md) | Raw pointers, calling unsafe functions |
| **FFI** | [Day 38](Day31-45/38.FFI/README.md) | Calling C code from Rust |
| **Macros** | [Day 30](Day16-30/30.AdvancedFnAndMacros/README.md) | Declarative macros (`macro_rules!`) |
| **Procedural Macros** | [Day 37](Day31-45/37.ProceduralMacros/README.md) | Custom derive, attribute macros |
| **SIMD** | [Day 88](Day81-90/88.SIMD/README.md) | Single Instruction Multiple Data intrinsics |
| **Embedded** | [Day 85](Day81-90/85.EmbeddedRust/README.md) | `embedded-hal`, `no_std` environments |

## ⚪ 9. Tools & Engineering
| Concept | Day / Project | Description |
|:---|:---|:---|
| **Testing** | [Day 15](Day01-15/15.Testing/README.md) | Unit tests, Integration tests, doctests |
| **Cargo & Crates** | [Day 18](Day16-30/18.CargoAndCrates/README.md) | Workspaces, publishing, profiles |
| **Serialization** | [Day 32](Day31-45/32.Serialization_Serde/README.md) | `Serde` (JSON, YAML, etc.) |
| **Logging & Tracing** | [Day 42](Day31-45/42.Tracing/README.md) | `log`, `tracing` ecosystem |
| **CLI Apps** | [Day 44](Day31-45/44.Clap/README.md) | `clap` for argument parsing |
| **Docker** | [Day 75](Day61-80/75.Docker/README.md) | Multistage builds, containerization |
| **CI/CD** | [Day 89](Day81-90/89.CICD/README.md) | GitHub Actions workflows |
| **Profiling** | [Day 86](Day81-90/86.Profiling/README.md) | Flamegraphs, benchmarking |
