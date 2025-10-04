// Test file for control flow statements

// Basic if-else tests
var test1 = 5;
if (test1 > 3) {
    print "test1 passed";
} else {
    print "test1 failed";
}

if (test1 < 3) {
    print "test2 failed";
} else {
    print "test2 passed";
}

// If without else
var test3 = 10;
if (test3 == 10) {
    print "test3 passed";
}

// Basic while loop
var counter = 3;
while (counter > 0) {
    print counter;
    counter = counter - 1;
}

// While loop with condition that starts false
var falseCounter = 0;
while (falseCounter > 5) {
    print "this should not print";
    falseCounter = falseCounter + 1;
}

// Basic for loop
for (var i = 1; i <= 3; i = i + 1) {
    print i;
}

// For loop with no initializer
var j = 5;
for (; j < 8; j = j + 1) {
    print j;
}

// For loop counting down
for (var k = 10; k > 7; k = k - 1) {
    print k;
}

// For loop with boolean conditions
for (var m = 1; m < 3; m = m + 1) {
    if (m == 1) {
        print "first iteration";
    } else {
        print "second iteration";
    }
}