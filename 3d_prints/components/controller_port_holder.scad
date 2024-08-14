$fa = 1;
$fs = .5;

module GX12Holder(depth = 5)
{
    width = 18;
    union()
    {
        difference()
        {
            cube(size = [ width, width, depth ]);
            translate([ width / 2, width / 2, -1 ]) linear_extrude(height = depth + 2) circle(r = 6);
            translate([ width / 2, width / 2, depth - 2 ]) linear_extrude(height = 3) circle(r = 7.5);
        }

        translate([ 0, (width / 2) - 5.5 - 2, 0 ]) cube(size = [ width, 2, depth - 2 ]);
        translate([ 0, (width / 2) + 5.5, 0 ]) cube(size = [ width, 2, depth - 2 ]);
    }
}
