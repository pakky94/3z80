$fa = 1;
$fs = .25;

height = 45;
width = 120;
depth = 45;
thickness = 2;
holes_indent = 5;
holes_h = 5;

module HoleSupport(h = 3)
{
    difference()
    {
        cube([ 6, 6, h ]);
        translate([ 3, 3, -1 ]) linear_extrude(h + 2) circle(1.5);
    }
}

union()
{
    difference()
    {
        cube([ width, height, depth ]);
        translate([ thickness, thickness, -1 ]) cube([ width - 2 * thickness, height - 2 * thickness, depth + 2 ]);
        translate([ -1, 10, 10 ]) cube([ thickness + 2, height - 20, 25 ]);
        translate([ width - thickness - 1, 10, 10 ]) cube([ thickness + 2, height - 20, 25 ]);
    }

    translate([ thickness, thickness, depth - holes_h ]) HoleSupport(holes_h);
    translate([ thickness, height - thickness - 6, depth - holes_h ]) HoleSupport(holes_h);
    translate([ width - thickness - 6, thickness, depth - holes_h ]) HoleSupport(holes_h);
    translate([ width - thickness - 6, height - thickness - 6, depth - holes_h ]) HoleSupport(holes_h);

    translate([ -6, 0, 0 ]) HoleSupport(3);
    translate([ -6, height - 6, 0 ]) HoleSupport(3);
    translate([ width, 0, 0 ]) HoleSupport(3);
    translate([ width, height - 6, 0 ]) HoleSupport(3);
}
