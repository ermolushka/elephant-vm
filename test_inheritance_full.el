// Comprehensive inheritance test

print "=== Basic Inheritance ===";
class Animal {
    speak() {
        print "Animal sound";
    }

    move() {
        print "Moving...";
    }
}

class Dog < Animal {
    speak() {
        print "Woof!";
    }

    bark() {
        print "Bark bark!";
    }
}

var dog = Dog();
dog.speak();  // Overridden method
dog.move();   // Inherited method
dog.bark();   // New method

print "";
print "=== Multi-level Inheritance ===";
class Puppy < Dog {
    play() {
        print "Playing with toy!";
    }
}

var puppy = Puppy();
puppy.speak();  // From Dog (which overrides Animal)
puppy.move();   // From Animal (inherited through Dog)
puppy.bark();   // From Dog
puppy.play();   // From Puppy

print "";
print "=== Field Inheritance ===";
class Vehicle {
    getType() {
        print "Vehicle";
    }
}

class Car < Vehicle {
    getType() {
        print "Car";
    }
}

var car = Car();
car.brand = "Toyota";
car.year = 2024;
car.getType();
print car.brand;
print car.year;

print "";
print "=== Test Complete ===";
