// Test super keyword
class Animal {
    speak() {
        print "Animal sound";
    }
}

class Dog < Animal {
    speak() {
        print "Before: Woof!";
        super.speak();
        print "After: Woof!";
    }
}

var dog = Dog();
dog.speak();
// Should print:
// Before: Woof!
// Animal sound
// After: Woof!
