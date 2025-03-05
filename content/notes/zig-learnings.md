---
title: Zig Learnings
date: 11 December 2024
---

This is a collection of my learnings about Zig. I am still a very newb in low level so I felt I should document some good learnings here.

### Optional Type 

An optional type in Zig is a way to represent a value that might or might not exist. It's like a box that can either contain something (a value of type `T`) or be empty `(null)`. For example:

+ If `T` is an integer `(i32)`, then `?i32` means "this could be an integer or nothing."
+ If `T` is a pointer `(*u8)`, then `?*u8` means "this could be a pointer or nothing."

```zig
var ptr: ?*i32 = null; // ptr is stored as 0x0
ptr = &some_integer;   // ptr now holds the address of some_integer
```
+ The optional pointer takes up the same amount of memory as a regular pointer (e.g., 8 bytes on a 64-bit system)

```zig
// the value may be byte or null
var maybe : ?u8 = null;	// prefix with ? mark  
// default value is null

maybe = 10;

var known = maybe.?; // if maybe is null, this will throw an error

var the_value = maybe orelse 0; // if maybe == null : return 0
// ternary in zig
var value = if (maybe) |b| b else 0;
```
+ `?` - Used to denote an option value while declaring a variable. It can also be used to unwrap an optional value.
+ `orelse` - Used to provide a default value if the optional value is null.

For non-pointer types (like integers, structs, etc.), Zig uses a 	`tagged union` to represent the optional.

A tagged union is a way to store either one type of data or another, along with a "tag" to indicate which one is currently stored.

```zig
var num: ?i32 = null; // Tag = 0x00, Payload = uninitialized
num = 42;             // Tag = 0x01, Payload = 42

// memory layout pseudocode
const OptionalI32 = struct {
    is_null: u8,        // 1 byte (0x00 for null, 0x01 for value)
    _padding: [3]u8,    // 3 bytes of padding (to align the i32)
    value: i32,         // 4 bytes (the actual integer)
};
```

The memory layout of a `i32` is as follows:
+ 1 byte for the tag.
+ 3 bytes of padding (to align the i32 to 4 bytes).
+ 4 bytes for the i32 value.



```zig
if (maybe) |b| {
	std.debug.print("The value is {d}\n", .{b});
}
```

+ If the value of `maybe` exists, it will be assigned to `b` and the block will be executed. If you are not interested in the payload, you can ignore it by using `_`

```zig
const allocator = std.heap.page_allocator;
var ptr: ?*i32 = try allocator.create(i32);
ptr.?.* = 42; // Unwrap and assign a value
allocator.destroy(ptr); // Free the memory
```

+ If an optional is part of a heap-allocated object (like a struct or array), you need to manage its memory manually using an allocator.

### Blocks 

+ Blocks are expressions that return a value.
+ Blocks are enclosed in curly braces {} and can be used in many places, such as in expressions, loops, or even as standalone constructs.

```zig
const x: u8 = blk: {
	var y: u8 = 10;
	var z: u8 = 20;
	break :blk y + z;
}
```
+ When the compiler sees a block, it creates a new scope for the variables inside it (e.g., `y` and `z`). When the block ends, the memory for these variables is automatically reclaimed.


#### Switch Statement Syntax

```zig
switch (y) {
	0 ... 20 => std.debug.print("0 ... 20\\n", .{}),
	21,22,32 => std.debug.print("21,22,32\\n", .{}),

	// capturing the value of y
	32..50 => |z| std.debug.print("{d}\\n", .{z}),

	// blocks for complex code
	77 => {
		const x = 10;
		std.debug.print("{d}\\n", .{x});
	},

	// as long as it is comptime known
 blk: {
		const a = 1000;
		break :blk a;
	} => std.debug.print("{d}\\n", .{z}),

	// has to be an exhaustive case
	else => std.debug.print("NONE OF THE ABOVE\\n", .{}),
};
```

1. The compiler ensures that the switch statement is exhaustive, meaning all possible values of y are covered. If not, it requires an else case
2. Ranges: `0...20` matches any value between 0 and 20 **(inclusive)**
3. If the value of y and the cases are known at `comptime`, the compiler can optimize the switch statement by replacing it with a direct branch to the case or removing unreachable code.

### Enums and Unions

```zig
const Color = enum {
	Red  = 1, // default value
	Green, // 2
	Blue, // 3
	_,

	fn isRed(self: Color) bool {
		return self == .Red;
	}
};
```

+ `enum` - Enumerations are a way to group a set of related values together. They are used to store a fixed set of values, and to provide type safety at the same time. Each value in enum is called a `variant`.
+ Each variant can optionally have an explicit integer value. If no value is specified, Zig assigns values starting from 0 (or the last explicit value + 1).
+ `_` - The _ variant is a catch-all for any value not explicitly listed

<br>

+ `enums` are stored as integers. If all values fit in `u8`, it is stored as `u8` otherwise the compilers uses the smallest integer type possible.
+ `Color` is stored as a u8 because its values `(1, 2, 3)` fit in `1` byte
+ When you call a method on an enum, the compiler passes the enum value `(self)` to the method.

```zig 
const Number = union {
	int: u8,
	float: f64,	
}
```

+ `union` - Helpful for using memory efficiently when you know that the value will always be one of the possibilities. 
+ It seperates the space required of the biggest type in memory. It is like when both of the fields are using the same amount of memory but only one of them is active.
+ Unions can also have functions associated with them.
+ Accessing an inactive field results in undefined behavior.

```zig
const num = Number{ .int = 42 };
switch (num) {
    .int => |x| std.debug.print("int: {d}\n", .{x}),
    .float => |x| std.debug.print("float: {d}\n", .{x}),
}
```

+ To safely access the active field, you can use a switch statement.


```zig
const Token = union(enum) {
	keyword_else: void,
	keyword_if, // if no payload, assumed void
	digit: usize,

	fn is(self: Token, tag: std.meta.Tag(Token)) bool {
		return std.meta.activeTag(self) == tag;
	}
}
``` 

+ A tagged union is a union combined with an enum to track which field is active. This adds type safety and makes it easier to work with unions.

+ Token is stored as:
	1. 1 byte for the tag (enum).
  1. 8 bytes for the payload (size of usize).
  2. Total size: 9 bytes (plus padding for alignment)

+ The compiler ensures that you only access the active variant. For example, if the active variant is digit, you cannot access keyword_else.

+ Methods can be defined on tagged unions, just like enums.

### Comptime

Anything marked `comptime` has to be known during the compile time. There cannot be any kind of computation or side effects at runtime. We can use this concept to create generics in pure zig

```zig
pub fn Point(comptime T: type) type {
	return struct {
		x: T,
		y: T,

		const Self = @This();

		pub fn new(x: T, y: T) Self {
			return .{ .x = x, .y = y };
		}

		pub fn distance(self: Self, other: Self) T {
			return std.math.sqrt((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y));
		}
	};
}
```

Now we can use this to create a point of any type

```zig
const P = Point(f64);
var p = P.new(1.0, 2.0);
var q = P.new(3.0, 4.0);

const P2 = Point(i32);
var p2 = P2.new(1, 2);
var q2 = P2.new(3, 4);
```

To do different things based on the type of the variable: we use `@typeInfo`

```zig
pub fn difference(self: Self, other: Self) f64 {
	const diffx: f64 = switch (@typeInfo(T)) {
		.int => @as(f64, @floatFromInt(self.x)) - @as(f64, @floatFromInt(other.x)),
		.float => self.x - other.x,
		else => @compileError("Unsupported type"),
	};

	const diffy: f64 = switch (@typeInfo(T)) {
		.int => @as(f64, @floatFromInt(self.y)) - @as(f64, @floatFromInt(other.y)),
		.float => self.y - other.y,
		else => @compileError("Unsupported type"),
	};

	return @sqrt(diffx * diffx + diffy * diffy);
}
``` 