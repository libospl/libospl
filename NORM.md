# Our rust norm;
--------------------------------------------------

This norm aims to make a code that is coherent and readable to other contributors.  
You should follow these rules as much as you can.  
Not following formatting rules will likely make your pull request rejected.  
Doing forbidden things will make us reject your pull request.  
If you have any question or remark about this norm, open an issue. We will read it carefully.

## Part 1: naming
Naming variables, functions names and other stuff like file names must follow these guidelines:

* Names must be clear and in english and as explicit as possible

### Variables
* Only lower case ascii characters allowed
* Words must be separated with underscore '\_' (ex: `database_path`)
* You can use shortcuts for the names if they are explicit enough (like `nb_thing` but not `nb_thg`)
* Every declaration of a variable must be used

### Structures
* Only ascii characters allowed
* Words are in CamelCase (ex: CreateDirectory)
* Exception can be made if the structure has no implementation

## Part 2: formatting{}
The code itself should respect a few things:

* You must indent the code with . ___tabulations___ not _spaces_.
* Try to avoid making functions over 30-35 lines. Try to _split_ the code **as most as you can**. Avoid making functions that can be done ___reéusable___.
* Try to avoid writing lines that are larger than 100 columns. Do not forget that tabulations are 4 columns large.
* Try to get the smallest form of your conditions. Smaller conditions are often easier to understand and optimized.
* Only one instruction per line.
* An empty line is an _empty_ line. No spaces, no tabulations. Empty.
* Declaring variable on the top of the function is often clearer but not mandatory.
* Try to space out your code sensibly. Avoid using more than one empty line.
* The end of a line should be empty. _Empty_ like empty lines.
* You must put the curly braces at the next line:

When cargo fmt will be stable enough, we will implement the rustfmt.toml file to make life easier.

```rust
// this is NOT right
fn main() -> i32{
	0
}

// this is right
fn main() -> i32
{
	0
}

```

## Part 3: functions()

* Avoid to put more than 4 parameters in a function.  
  If your function needs more than 4 parameters then it probably can be split  
  or you probably can create data structures handling some of thoses parameters.
* For spacing-out reason please add a single empty line after each function:

```rust
// This is NOT right
fn my_function(s1: String, s2: String, s3: String, s4: String, s5: String) -> i32
{
	1
}
fn my_second() -> i32
{
	2
}

// This is right
fn my_function(s1: String, s2: String)
{
	1
}

fn my_second(s1: String, s2 String)
{
	2
}

```
* Try to make a little comment over each function to explain it (if it's not clear enough)
* You can also add what parameters it takes and what it returns
* We are using cargo doc to generate the documentation so please use the cargo doc format.
* Public function should always have a documentation
* Always try to add idiomatic examples
* You can get inspiration [here](https://blog.guillaume-gomez.fr/articles/2020-03-12+Guide+on+how+to+write+documentation+for+a+Rust+crate#Writing-a-good-documentation-block)

```rust
/// Returns the given parameter
///
/// This function returns the integer given as first parameter
///
/// # Examples:
///
/// Basic usage:
/// ```
/// let param1 = my_superb_function(5);
/// ```
fn my_superb_function(param1: u32) -> u32
{
	param1
}
```

## Part 4: files.rs

* File names must be explicit about what they contain.  
  If it contains only a struct definition, it should have the struct name as its own name.
  If it conatins only one function, it should have the function name as its own name.
* A file should never contain functions that are not related one to each other.
* Please write the short licence at the top of the file

## Short licence

You must add the short licence at the top of new files.

```rust
/*	libospl - Open Source Photo Library
	an opensource and multiplateform photo library management that can be used
	to store and sort all your photos.
	Copyright (C) 2019-2022 Angelo Frangione

	This program is free software; you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation; either version 2 of the License, or
	(at your option) any later version.

	This program is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License along
	with this program; if not, write to the Free Software Foundation, Inc.,
	51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
*/
```
