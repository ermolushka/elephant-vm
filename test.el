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
