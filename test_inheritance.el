// Test basic inheritance
class Animal {
    speak() {
        print "Some sound";
    }

    move() {
        print "Moving...";
    }
}

class Dog < Animal {
    speak() {
        print "Woof!";
    }
}

var dog = Dog();
dog.speak();  // Should print "Woof!"
dog.move();   // Should print "Moving..." (inherited from Animal)

print "---";

// Test multiple levels of inheritance
class Puppy < Dog {
    playful() {
        print "Playing!";
    }
}

var puppy = Puppy();
puppy.speak();    // Should print "Woof!" (from Dog)
puppy.move();     // Should print "Moving..." (from Animal)
puppy.playful();  // Should print "Playing!" (from Puppy)
