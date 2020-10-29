#![allow(dead_code)]
#![allow(unused_imports)]

mod obj;
mod space;

use obj::craft::Craft;
use obj::location::Location;
use space::point::Point;
use space::quaternion::Quaternion;
use space::vector::Vector;

/*
To-do:
Redefine equality / operations to round to correct sig figs
*/

/*
Overall algorithm:

Are evasive manoeuvres necessary?
Repeats always:
1. Update enemy and self location
2. Check if self location within enemy sphere
3. If yes:
    3.1. If within closer radius to enemy plane:
        3.1.1. Place waypoint behind enemy current location (+ certain margin?)
    3.2. Check if self location on surface of spherical sector projected in front of enemy
    3.3. If yes:
        3.3.1. Determine if enemy path intersects UAV path at same time (+/- a margin)
        3.3.2. If yes:
            3.3.2.1. Evasive manoeuvres
        3.3.3. Else:
            3.3.3.1. Remove evasive waypoints from path
    3.4. Else:
        3.4.1. Remove evasive waypoints from path
4. Else:
    4.1. If within closer radius to enemy plane:
        4.1.1. Place waypoint behind enemy current location (+ certain margin?)
    4.2. Remove evasive waypoints from path


Evasive Manoeuvres (Not yet finalised)
1. If > certain dist from enemy plane:
    1.1. Move towards enemy
    1.2. If @ same altitude as enemy:
        1.2.1. Move up
    1.3 Place waypoint

*/
