```
cargo build
```

The program defines an elliptic curve with specific parameters (a, b, and the prime field). This curve is used throughout the operations.

# Performing Point Addition and Doubling:

It creates two elliptic curve points (point and second_point) with specified coordinates.
It performs several addition operations with these points:
It doubles the point.
It adds point to second_point.
It adds point to the point at infinity.
It adds point to its inverse.
For each operation, it prints the resulting coordinates of the resulting point.

```
cargo add
```

results:

```
Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/ecc-operations add`
point at infinity:0,1,0
doubling
x3:9,y3:0,z3:8
adding
x3:10,y3:6,z3:1
adding point at infinity
x3:5,y3:1,z3:1
adding inverse
x3:0,y3:1,z3:0
```

### Performing Point Multiplication:

Like in the add mode, the program defines an elliptic curve with specific parameters.

It creates an elliptic curve point (point) with specified coordinates.
It multiplies this point by the scalar value 70.
During the multiplication process, it prints the value of n (the scalar being multiplied) at each step.
After multiplication, it prints the coordinates of the resulting point.

```
cargo run multiply
```

reults

```
35
17
8
4
2
1
0
x3:9,y3:0,z3:8
```
