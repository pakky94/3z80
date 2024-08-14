module C8Plug(depth = 5)
{
    height = 40;
    width = 20;

    h2 = 20.1;
    w2 = 12.8;

    union()
    {
        difference()
        {
            cube(size = [ width, height, depth ]);
            translate([ (width - w2) / 2, (height - h2) / 2, depth - 4 ]) cube(size = [ w2, h2, 5 ]);

            translate([ (width - 15.2) / 2, (height - 32.1) / 2, -3 ]) cube([ 15.2, 32.1, depth ]);

            translate([ width / 2, (height / 2) - 13.25, -1 ]) linear_extrude(depth + 2) circle(1.5);
            translate([ width / 2, (height / 2) + 13.25, -1 ]) linear_extrude(depth + 2) circle(1.5);
        }
    }
}
