var a = 2;
if (a > 0) {
    print "more";
} else {
    print "less";
}

var c = 5;
while (c > 0) {
    print c;
    c = c - 1;
    var dd = 5;
    while (dd > 0) {
        print dd;
        dd = dd - 1;
    }
}

for (var i = 1; i < 4; i = i + 1) {
    print i;
    for (var j = 1; j < 4; j = j + 1) {
        print i + j;
    }
}
