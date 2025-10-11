var b = 14;
{
    var a = 2;
    print a;
    print b;
    {
        var c = 1331;
        print c;
        print a;
        print b;
    }
    {
    var a = "outer";
    {
        var a = a;
    }
    }
}

if (4 > 2) {
    print "if works";
}

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