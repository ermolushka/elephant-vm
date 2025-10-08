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
