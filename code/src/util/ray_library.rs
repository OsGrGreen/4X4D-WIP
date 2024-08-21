use glam::Vec3;


pub fn ray_plane_intersect(p0: Vec3,d:Vec3,q:Vec3,n:Vec3) -> Vec3{

    /*
    P0: The starting point of the ray 
    d: The direction vector of the ray 
    Q: A point on the plane 
    n: The normal vector of the plane 

    */

    let denom = n.dot(d);

    let t = (n.dot(q-p0)) / denom;

    let intersection_point = p0 + t * d;

    return intersection_point;

}