# insrcdata
*embed your data*


insrcdata is a source code generator for embedding tabular data as a static array

## Why embedding data in code

Data is at the core of software. 
A large part of the data must be provided by the editor of the software.
Integrating it statically into source code has several advantages :

**Access to embedded data is fast** since it avoids the performance overhead of reading from file or database.

**Access is reliable** because the compiler can check data integrity at compile time
and will avoid  runtime errors during data access.

**The program is easier to deploy** by having a single file to distribute. 
And you have more control over the coherence of code and data : the new code will never break because of old data.

**Software development is faster** because less code is needed to get data 
and because debugging is far easier.

## Why using insrcdata

Programmers can write anything by hand, but *innexata* will make their lives easier:

Most of the time, domain experts use their own tools to fashion the data to be used by the program.
Programmers will have to rewrite the output of these tools in their programming language syntax. 
This task, tedious and error-prone, will be avoided with *insrcdata*.

Some features are not that easy to write by hand. For example, you will get indexed searches 
and table joins almost for free.

The code can be generated for several languages : Rust and C today, other targets are planned.

## Usage

*insrcdata* is a command line tool that will generate source code from data in [.csv](Comma-separated values) file 
and configuration in a [.toml](https://toml.io)  file

You will find several uses in the sample directory. 
We suggest to start with the helloworld tutorial

### Installing

You can build *insrcdata* from source. 
You will need [Rust](https://www.rust-lang.org/) compiler suite.

In a terminal, go at the root of insrcdata source and type :
```console
cargo build
```
you will find *insrcdata* executable  in the target/debug subdirectory.


### Data source csv file
You will use one file per table.
The first line of the file is a header that defines the names of the columns.  
The following lines are the rows of the array that will implement the table. 

### Configuration file
First level sections correspond to the tables defined in the project.
Second level sections are the columns of the tables.
More information can be found in template.toml in the sample directory.
