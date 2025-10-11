// Test basic class definition and instantiation
class Person {
    greet() {
        print "Hello, I'm a person!";
    }
}

// Create an instance
var person = Person();
person.greet();

// Test class with fields
var person2 = Person();
person2.name = "Alice";
print person2.name;
