// Function declaration
fun hello() {
    print "Hello, World!";
}

// Function calling
hello();

// Functions with return values
fun add() {
    return 42;
}

var result = add();
print result; // Prints: 42

// Recursive functions
var counter = 0;

fun countdown() {
    if (counter > 0) {
        print counter;
        counter = counter - 1;
        countdown();
    } else {
        print "Done!";
    }
}

counter = 5;
countdown();

fun test(n) {
    print n;
}

// Test function with multiple arguments
fun add(a, b) {
    return a + b;
}

// Test function with single argument and return
fun square(x) {
    return x * x;
}

// Test function with no arguments (should still work)
fun get_answer() {
    return 42;
}

// Test function with many arguments
fun complex_calc(a, b, c, d) {
    return a + b * c - d;
}

// Test function calls
test(100);
print add(5, 3);
print square(4);
print get_answer();
print complex_calc(1, 2, 3, 4);