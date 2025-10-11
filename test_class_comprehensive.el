// Comprehensive class tests for Elephant VM

// Test 1: Basic class definition and instantiation
print "Test 1: Basic class";
class Animal {
    speak() {
        print "Some generic animal sound";
    }
}

var animal = Animal();
animal.speak();

// Test 2: Instance fields
print "Test 2: Instance fields";
class Dog {
    bark() {
        print "Woof!";
    }
}

var dog1 = Dog();
dog1.name = "Buddy";
dog1.age = 3;

var dog2 = Dog();
dog2.name = "Max";
dog2.age = 5;

print dog1.name;
print dog1.age;
print dog2.name;
print dog2.age;
dog1.bark();

// Test 3: Multiple methods
print "Test 3: Multiple methods";
class Calculator {
    add() {
        print "Adding...";
    }

    subtract() {
        print "Subtracting...";
    }

    multiply() {
        print "Multiplying...";
    }
}

var calc = Calculator();
calc.add();
calc.subtract();
calc.multiply();

// Test 4: Field modification
print "Test 4: Field modification";
class Counter {
    increment() {
        print "Incrementing";
    }
}

var counter = Counter();
counter.value = 0;
print counter.value;
counter.value = 10;
print counter.value;
counter.increment();

// Test 5: Multiple instances with independent fields
print "Test 5: Independent instances";
class Person {
    greet() {
        print "Hello!";
    }
}

var person1 = Person();
var person2 = Person();
person1.name = "Alice";
person2.name = "Bob";
print person1.name;
print person2.name;
person1.greet();
person2.greet();

print "All tests completed!";
