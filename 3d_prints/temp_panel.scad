use <components/controller_port_holder.scad>
use <components/mx_switch_mount.scad>
use <components/power_plug.scad>

$fa = 1;
$fs = .3;

depth = 5;
height = 45;
width = 120;
holes_indent = 5;

power_pos = 83;
reset_pos = 55;

union()
{
    difference()
    {
        cube([ width, height, depth ]);

        // GX12 cutouts
        translate([ 14, (height - 16) / 2, -1 ]) cube([ 16, 16, depth + 2 ]);
        translate([ 34, (height - 16) / 2, -1 ]) cube([ 16, 16, depth + 2 ]);

        // mount holes
        translate([ holes_indent, holes_indent, -1 ]) linear_extrude(height = depth + 2) circle(1.5);
        translate([ holes_indent, height - holes_indent, -1 ]) linear_extrude(height = depth + 2) circle(1.5);
        translate([ width - holes_indent, holes_indent, -1 ]) linear_extrude(height = depth + 2) circle(1.5);
        translate([ width - holes_indent, height - holes_indent, -1 ]) linear_extrude(height = depth + 2) circle(1.5);

        // power plug
        translate([ width - 29, (height - 38) / 2, -1 ]) cube([ 18, 38, depth + 2 ]);

        // power switch
        translate([ power_pos, height / 2 - 10, -1 ]) linear_extrude(height = depth + 2) circle(3);
        // power led
        translate([ power_pos, height / 2 + 10, -1 ]) linear_extrude(height = depth + 2) circle(4);

        // reset button
        translate([ reset_pos, (height - 20) / 2, -1 ]) cube([ 20, 20, depth + 2 ]);
    }

    translate([ 13, (height - 18) / 2, 0 ]) GX12Holder(depth);
    translate([ 33, (height - 18) / 2, 0 ]) GX12Holder(depth);

    translate([ width - 30, (height - 40) / 2, 0 ]) C8Plug(depth);

    translate([ reset_pos - 1, (height - 22) / 2, 0 ]) MxMount(depth);
}
