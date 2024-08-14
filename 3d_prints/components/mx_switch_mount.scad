module MxMount(depth = 5)
{
    difference()
    {
        cube([ 22, 22, depth ]);
        translate([ 4, 4, -1 ]) cube([ 14, 14, depth + 2 ]);
        translate([ 2, 2, -1 ]) cube([ 18, 18, depth - .5 ]);
    }
}
