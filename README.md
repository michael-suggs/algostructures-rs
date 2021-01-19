# AlgoStructures (Rust)

A collection of algorithms and data structures, all implemented in pure Rust. These are by no means optimal (nor necessarily optimized in any way, shape, or form), as this is primarily an educational endeavor for me in my pursuit of learning Rust.

The vast majority of algorithms and whatnot found herein are implemented based on specifications found in a few textbooks. As this progresses, I may add additional sources to provide code structures not found in these.

A few of the aforementioned references I have used are as follows:

- [Data Structures and Algorithms in Java (6e), Goodrich et al.](https://www.wiley.com/en-us/Data+Structures+and+Algorithms+in+Java%2C+6th+Edition-p-9781118771334)

- [Introduction to Algorithms (3e) (CLRS), Cormen et al.](https://mitpress.mit.edu/books/introduction-algorithms)

## Repository Structure

Typical Cargo library crate structure is followed. Sections are broken down into broad categories as modules, which may contain submodules for more intricate or involved feature sections. Top-level module-definition files (e.g. [graph.rs](src/graph.rs)) scope in all module and submodule components, and will typically contain module-wide shared features and configurations (such as basic graph definitions). As is idiomatic, unit tests are found in their respective modules.

At present, the topmost modules are [algorithms](src/algorithms.rs), [data structures](src/data_structures.rs), and [graphs](src/graph.rs).
